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
