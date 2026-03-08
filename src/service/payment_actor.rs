use crate::model::account::Account;
use crate::model::client_id::ClientId;
use crate::model::my_result::MyResult;
use crate::model::transaction::Transaction;
use std::collections::HashMap;
use tokio::sync::mpsc::Receiver;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub struct ActorId(pub u16);

pub struct PaymentActor {
    pub id: ActorId,
    accounts: HashMap<ClientId, Account>,
    receiver: Receiver<Transaction>,
}

impl PaymentActor {
    pub fn new(id: ActorId, receiver: Receiver<Transaction>) -> Self {
        PaymentActor {
            id,
            accounts: HashMap::new(),
            receiver,
        }
    }

    pub async fn run(mut self) -> Vec<Account> {
        while let Some(txn) = self.receiver.recv().await {
            self.process(txn).unwrap_or_default()
        }
        let state: Vec<Account> = self.accounts.into_values().collect();
        state
    }

    fn process(&mut self, txn: Transaction) -> MyResult<()> {
        let account = self.get_account(txn.client());

        match txn {
            Transaction::Deposit(dep) => account.deposit(dep.tx, dep.amount),
            Transaction::Withdrawal(with) => account.withdraw(with.amount),
            Transaction::Dispute(disp) => account.dispute(disp.tx),
            Transaction::Resolve(resolve) => account.resolve(resolve.tx),
            Transaction::Chargeback(chg) => account.chargeback(chg.tx),
        }
    }

    fn get_account(&mut self, id: ClientId) -> &mut Account {
        self.accounts.entry(id).or_insert_with(|| Account::new(id))
    }
}
