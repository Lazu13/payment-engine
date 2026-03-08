# payment-engine
Payment-engine that supports deposits, withdrawal, dispute, resolve and chargeback

## Logic
Application takes a path to a CSV file, then iterates over records, deserializing into `TransactionDTO` record and mapping into `Transaction`.
Application ignores all of the broken records (deserialization, mapping).

Next, `PaymentEngine` is called that is responsible for forwarding the `Transaction` to right `PaymentActor`. Multiple actors are spawned to ensure parallel processing.
Important note here is that there is maximum number of all possible actors to be spawned. This is to ensure no memory overflow happen. `get_actor_id` method specifies algorithm to ensure sequential processing per client.

`PaymentActor` processes messages sequentially following defined logic. 

At the end, after having processed all the messages from CSV file, applications collects all final states and outputs them to stdout. 

## Validation
- only positive amount values
- only decimals with up to 4 decimal places

## Note
- only deposits can be disputed

## Usage
```cargo
cargo run -- transaction.csv
```

## TODO
1. handle parallel creation of payment actor
2. handle txn ids checks (deduplication)