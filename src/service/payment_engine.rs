use crate::model::account::Account;
use crate::model::client_id::ClientId;
use crate::model::my_error::MyError;
use crate::model::my_result::MyResult;
use crate::model::transaction::Transaction;
use crate::service::payment_actor::{ActorId, PaymentActor};
use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

pub struct SenderHandle {
    sender: mpsc::Sender<Transaction>,
    handle: JoinHandle<Vec<Account>>,
}

impl SenderHandle {
    pub fn new(sender: mpsc::Sender<Transaction>, handle: JoinHandle<Vec<Account>>) -> Self {
        SenderHandle { sender, handle }
    }
}

pub struct PaymentEngine {
    max_buffer: usize,
    max_actor_count: u16,
    actors: HashMap<ActorId, SenderHandle>,
}

impl PaymentEngine {
    pub fn new(max_buffer: usize, max_actor_count: u16) -> Self {
        PaymentEngine {
            max_buffer,
            max_actor_count,
            actors: HashMap::with_capacity(max_actor_count as usize),
        }
    }

    pub async fn shutdown(mut self) -> MyResult<Vec<Account>> {
        let handlers: Vec<_> = self.actors
            .drain()
            .map(|(_, actor)| actor.handle)
            .collect();

        let mut accounts = Vec::new();

        for handler in handlers {
            let result: Vec<Account> = handler
                .await
                .map_err(|_| MyError::InternalError)?;
            accounts.extend(result);
        }

        Ok(accounts)
    }

    pub async fn process(&mut self, txn: Transaction) -> MyResult<()> {
        let sender_handle = self.get_actor(txn.client());

        sender_handle
            .sender
            .send(txn)
            .await
            .map_err(MyError::ProcessingError)
    }

    fn get_actor(&mut self, id: ClientId) -> &SenderHandle {
        let actor_id = self.get_actor_id(id);

        self.actors
            .entry(actor_id)
            .or_insert_with(|| Self::spawn_actor(self.max_buffer, actor_id))
    }

    fn get_actor_id(&self, id: ClientId) -> ActorId {
        ActorId(id.0 % self.max_actor_count)
    }

    fn spawn_actor(max_buffer: usize, id: ActorId) -> SenderHandle {
        let (tx, rx) = mpsc::channel::<Transaction>(max_buffer);

        let handle = tokio::spawn(async move {
            let actor = PaymentActor::new(id, rx);
            let state: Vec<Account> = actor.run().await;
            state
        });

        SenderHandle::new(tx, handle)
    }
}
