# whirlpool-tx-replayer

## Requirements
### Software
- Rust (cargo + rustc)
- Docker (docker)
- MariaDB CLI (mysql)

### Disk
- 10 GB for MariaDB (Docker)
- some GB for rust compiler

### Memory
- 1 GB for MariaDB
- 0.5 GB for whirlpool-tx-replayer

## Setup
### build whirlpool-tx-replayer
debug build is much slower than release build, so please use debug build for the debugging only.
```
cargo build --release
```

### setup Transaction DB with exported data
#### startup MariaDB container on Docker

#### download exported data


#### import exported data


## Test Run
```
time cargo run --release
```