We are building a distributed KV store in Rust. We are following a Shared-Nothing / Thread-per-Core architecture.

Every component must be !Send and !Sync where possible. Avoid Arc, Mutex, and RwLock.

Efficiency and L1/L2 cache locality are the highest priorities!!

Ask before adding dependencies. Avoid extra dependencies unless they are absolutely necessary.