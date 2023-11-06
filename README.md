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

## Build whirlpool-tx-replayer
debug build is much slower than release build, so please use debug build for the debugging only.
```
cargo build --release
```

## Setup Transaction DB with exported data
### startup MariaDB container on Docker
download the latest mariadb image.
```
docker image pull mariadb
```

start the container named txdb
- root password: password
- port: 3306
```
docker container run --name txdb -e MYSQL_ROOT_PASSWORD=password -dp 3306:3306 mariadb
```

check if the container is running
```
docker container ls
```
```
CONTAINER ID   IMAGE     COMMAND                   CREATED         STATUS         PORTS                    NAMES
b9d69b45ade8   mariadb   "docker-entrypoint.s…"   6 seconds ago   Up 5 seconds   0.0.0.0:3306->3306/tcp   txdb
```

### download exported data
- approx 1.2 GB
- minimum slot: 214824664 (September 01, 2023 00:00:00 +UTC)
- maximum slot: 224369122 (October 18, 2023 03:30:50 +UTC)
- all tables, views and stored procedures(functions) are included
- [DDL for tables and stored procedures](https://github.com/everlastingsong/sedimentology/blob/main/src/sql/definition.sql)
- [DDL for views](https://github.com/orca-so/whirlpool-tx-replayer/blob/main/sql/view.sql)
```
curl -L -o whirlpool-transactions-214824664-224369122.dump.sql.gz https://www.dropbox.com/scl/fi/eyokubd0h1m967w7skwsj/whirlpool-transactions-214824664-224369122.dump.sql.gz?rlkey=z4pd383bbzfq21yettv9dru9e&dl=1
```

### import exported data
create empty database named "whirlpool"
```
mysqladmin -u root -p -h localhost -P 3306 create whirlpool
```

import data into "whirlpool" database
```
gunzip -c whirlpool-transactions-214824664-224369122.dump.sql.gz | mysql -u root -p -h localhost -P 3306 whirlpool
```

### sanity check
```
mysql -u root -p -h localhost -P 3306 whirlpool -e "SELECT COUNT(*) as slots, MIN(slot) as oldest, MAX(slot) as latest FROM slots"
```
```
+---------+-----------+-----------+
| slots   | oldest    | latest    |
+---------+-----------+-----------+
| 9274457 | 214824664 | 224369122 |
+---------+-----------+-----------+
```

```
mysql -u root -p -h localhost -P 3306 whirlpool -e "SELECT table_name, table_rows FROM information_schema.tables WHERE table_schema = 'whirlpool' AND table_name LIKE 'ixs%' ORDER BY table_rows DESC"
```
```
+-----------------------------------------+------------+
| table_name                              | table_rows |
+-----------------------------------------+------------+
| ixsSwap                                 |    6546276 |
| ixsUpdateFeesAndRewards                 |    1753725 |
| ixsIncreaseLiquidity                    |    1600220 |
| ixsCollectReward                        |    1593065 |
| ixsCollectFees                          |    1204790 |
| ixsDecreaseLiquidity                    |     844936 |
| ixsCollectProtocolFees                  |      84400 |
| ixsTwoHopSwap                           |      45738 |
| ixsClosePosition                        |      31164 |
| ixsOpenPositionWithMetadata             |      16608 |
| ixsOpenPosition                         |      14600 |
| ixsOpenBundledPosition                  |       4706 |
| ixsCloseBundledPosition                 |       4690 |
| ixsInitializeTickArray                  |       1917 |
| ixsInitializePool                       |        207 |
| ixsSetRewardEmissions                   |         60 |
| ixsInitializePositionBundle             |         23 |
| ixsDeletePositionBundle                 |         16 |
| ixsInitializeReward                     |          7 |
| ixsSetFeeRate                           |          2 |
| ixsSetFeeAuthority                      |          0 |
| ixsInitializePositionBundleWithMetadata |          0 |
| ixsSetDefaultFeeRate                    |          0 |
| ixsInitializeConfig                     |          0 |
| ixsSetProtocolFeeRate                   |          0 |
| ixsSetRewardEmissionsSuperAuthority     |          0 |
| ixsAdminIncreaseLiquidity               |          0 |
| ixsInitializeFeeTier                    |          0 |
| ixsSetCollectProtocolFeesAuthority      |          0 |
| ixsSetDefaultProtocolFeeRate            |          0 |
| ixsSetRewardAuthority                   |          0 |
| ixsSetRewardAuthorityBySuperAuthority   |          0 |
+-----------------------------------------+------------+
```


## Test Run
```
time cargo run --release
```
```
...
...
[15:56:25, 100.42%] processing slot = Slot { slot: 215150060, block_height: 197499197, block_time: 1693667197 } ...
  replaying instruction = updateFeesAndRewards ...
  replaying instruction = collectFees ...
  replaying instruction = swap ...
  replaying instruction = increaseLiquidity ...
[15:56:25, 100.43%] processing slot = Slot { slot: 215150061, block_height: 197499198, block_time: 1693667197 } ...
[15:56:25, 100.44%] processing slot = Slot { slot: 215150062, block_height: 197499199, block_time: 1693667197 } ...
  replaying instruction = swap ...
[15:56:25, 100.44%] processing slot = Slot { slot: 215150063, block_height: 197499200, block_time: 1693667198 } ...
[15:56:25, 100.45%] processing slot = Slot { slot: 215150064, block_height: 197499201, block_time: 1693667199 } ...
  replaying instruction = swap ...
  replaying instruction = swap ...
[15:56:25, 100.46%] processing slot = Slot { slot: 215150065, block_height: 197499202, block_time: 1693667199 } ...
[15:56:25, 100.46%] processing slot = Slot { slot: 215150066, block_height: 197499203, block_time: 1693667199 } ...
  replaying instruction = swap ...
[15:56:25, 100.47%] processing slot = Slot { slot: 215150067, block_height: 197499204, block_time: 1693667199 } ...

real    4m34.403s
user    2m33.990s
sys     0m43.366s
```

- will take several minutes (~10min, CPU intensive)
- will load ``tests/input-snapshot/whirlpool-snapshot-215135999.csv.gz`` as start point
- will do replay based on whirlpool instructions extracted from the transaction DB
- will save some snapshot into ``tests/output-snapshot``
- will stop at slot 215150000+
- you can verify the correctness of the replaying by comparing snapshots between ``tests/output-snapshot`` and ``tests/target-snapshot``. please see [README](https://github.com/orca-so/whirlpool-tx-replayer/blob/main/tests/output-snapshot/README.md) for details.


## TODO
### ~~Performance tuning~~
~~Now I believe that it can process 50 slots per seconds in average, and it is x20 faster than real validators.~~
~~But there is obvious hot spot and it is whirlpool program compilation everytime to execute transaction.~~

After eliminating program loading, replayer can process 2 days worth slots in 40 minutes. It is approx x70 replay performance! 🔥

### ~~Set Compute Budget~~
Allow instruction to use more compute budget.

### ~~Use 1.16.18~~
Now patch for solana-storage-proto can be removed.

### Support all instruction
The following instructions do not yet implement replay.
They are only rarely executed, and they are not technically difficult.

- InitializeConfig
- ~~InitializeFeeTier~~
- SetCollectProtocolFeesAuthority
- SetDefaultFeeRate
- SetDefaultProtocolFeeRate
- SetFeeAuthority
- ~~SetFeeRate~~
- SetProtocolFeeRate
- SetRewardAuthority
- SetRewardAuthorityBySuperAuthority
- SetRewardEmissionsSuperAuthority
- AdminIncreaseLiquidity

### Validation at instruction level
By performing the following verification before and after each instruction is executed, abnormal situations may be detected at an early stage.

- Accounts that should not exist do not exist
- Accounts that should exist do exist
- Token volume consistent with the transaction log has been transferred

### More performance tuning
- ConfirmedTransactionWithStatusMeta::encode in execute_transaction is the next hotspot
- Delete unused features such as token balance snapshots


### think: use RDBMS to store Whirlpool account snapshots
At the moment, simple gzipped csv files on Filesystem is used.

### think: direct use from Node.js (Typescript) similar to Bankrun
https://kevinheavey.github.io/solana-bankrun/


## Related works
### solana-snapshot-gpa
https://github.com/everlastingsong/solana-snapshot-gpa

input-snapshot and target-snapshot are extracted jito's snapshots using solana-snapshot-gpa.

### sedimentology
https://github.com/everlastingsong/sedimentology

The transaction DB used above is output of this project.

### whirlpool-tx-decoder
https://github.com/yugure-orca/whirlpool-tx-decoder

sedimentology uses this package to extract whirlpool instructions from transactions.

### solana-poc-framework
https://github.com/neodyme-labs/solana-poc-framework

``ReplayEnvironment`` is a modification of poc-framework's ``LocalEnvironment``. Because there were some points that were not suitable for replay, ``LocalEnvironment`` was copied and reworked into ``ReplayEnvironment``.
