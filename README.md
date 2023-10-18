# whirlpool-tx-replayer

<img width="765" alt="screenshot 2023-10-18 15 22 30" src="https://github.com/orca-so/whirlpool-tx-replayer/assets/109891005/ff52f804-132c-40ae-bf4b-0e89dc24dff9">

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
