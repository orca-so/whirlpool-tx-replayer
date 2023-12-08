# whirlpool-replayer

## Concept
Replaying the entire Solana would require a lot of resources, but for Whirlpool state alone, replay is possible with a snapshot of the account and all successful transactions associated with Whirlpool program.

Replay would allow for easy reproduction of the state at all points in time in the past.

``whirlpool-replayer`` library does not require a large amount of resources and is intended to be able to run even on a simple laptop. This library will give developers free access to Whirlpool history since its genesis.

<img width="765" alt="screenshot 2023-10-18 15 22 30" src="https://github.com/orca-so/whirlpool-tx-replayer/assets/109891005/ff52f804-132c-40ae-bf4b-0e89dc24dff9">

## Requirements
### Software
- Rust (cargo + rustc)

### Disk
- some GB for rust compiler
- ``whirlpool-replayer`` will download required data from cloud storage dynamically, so it can run without disk space.
  - But you can cache the data on the local disk to improve the performance and reduce the network traffic.
  - If you cache all data, the size will be upto 35 GB (at Dec'23).

### Memory
- 1 ~ 2 GB depending on the number of accounts and daily transaction realated to Whirlpool program

## Use whirlpool-replayer
You can use ``whirlpool-replayer`` library by importing it from GitHub repo directly.

```
whirlpool-replayer = { git = "https://github.com/orca-so/whirlpool-tx-replayer", package = "whirlpool-replayer" }
```

## Demo of whirilpool-replayer
This repository contains ``whirlpool-replay`` command implementation, and so you can try it as follows.
- Of course this command implementation depends on ``whirlpool-replayer`` library.
- The following command will just replay all whirlpool instruction at 2022/04/07.
- It outputs all instruction replayed and the details of swap instruction.
- ``data/sample_local_storage`` contains only data for 2022/04/07 and 2022/04/08.
```
$ cargo run --release -p whirlpool-replay data/sample_local_storage 20220407

...
processing slot: 128703624 (block_height=116654348 block_time=1649375977) ...
processing slot: 128703625 (block_height=116654349 block_time=1649375977) ...
  replayed instruction: swap
    tx signature: 4VXiFMADVXYWqp33sZRFr1n5Jq8MM9y9GAtTTvqzXCLLRLK8YjvWmGqEmT86oMX77kZnphWDowFu5WcAodiZSiyC
    pool: 65shmpuYmxx5p7ggNCZbyrGLCXVqbBR1ZD5aAocRBUNG (ts=128, fee_rate=2500)
      direction: A to B
      in/out: in=2000000000 out=236884278
      sqrt_price: pre=6356607777186099091 post=6356343655694204870
processing slot: 128703626 (block_height=116654350 block_time=1649375978) ...
processing slot: 128703627 (block_height=116654351 block_time=1649375979) ...
processing slot: 128703628 (block_height=116654352 block_time=1649375979) ...
...
processing slot: 128703657 (block_height=116654381 block_time=1649375997) ...
  replayed instruction: openPositionWithMetadata
  replayed instruction: increaseLiquidity
processing slot: 128703658 (block_height=116654382 block_time=1649375997) ...
  replayed instruction: setRewardEmissions
processing slot: 128703659 (block_height=116654383 block_time=1649375998) ...
```

If you know the remote storage endpoint, the following will work well, too.

```
$ cargo run --release -p whirlpool-replay <REMOTE STORAGE ENDPOINT> <YYYYMMDD>
```


## TODO
### ~~Performance tuning~~
~~Now I believe that it can process 50 slots per seconds in average, and it is x20 faster than real validators.~~
~~But there is obvious hot spot and it is whirlpool program compilation everytime to execute transaction.~~

After eliminating program loading, replayer can process 2 days worth slots in 40 minutes. It is approx x70 replay performance! 🔥

### ~~Set Compute Budget~~
Allow instruction to use more compute budget.

### ~~Use 1.16.18~~
Now patch for solana-storage-proto can be removed.

### ~~Support all instruction~~
The following instructions do not yet implement replay.
They are only rarely executed, and they are not technically difficult.

- ~~InitializeConfig~~
- ~~InitializeFeeTier~~
- ~~SetCollectProtocolFeesAuthority~~
- ~~SetDefaultFeeRate~~
- ~~SetDefaultProtocolFeeRate~~
- ~~SetFeeAuthority~~
- ~~SetFeeRate~~
- ~~SetProtocolFeeRate~~
- ~~SetRewardAuthority~~
- ~~SetRewardAuthorityBySuperAuthority~~
- ~~SetRewardEmissionsSuperAuthority~~
- ~~AdminIncreaseLiquidity~~

### ~~Switching program version dynamically based on a txid~~
We need to switch whirlpool program in src/programs/whirlpool.

### ~~Separate core and use crate~~

### WASM build for client use

### Handling new deployment release
We know when whirlpool program have been updated in the past exactly.
But we cannot know which slot and which tx index in the block the next version will be released.

### Validation at instruction level
By performing the following verification before and after each instruction is executed, abnormal situations may be detected at an early stage.

- Accounts that should not exist do not exist
- Accounts that should exist do exist
- Token volume consistent with the transaction log has been transferred

### More performance tuning
- ~~ConfirmedTransactionWithStatusMeta::encode in execute_transaction is the next hotspot~~
- Delete unused features such as token balance snapshots

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

### dev-null-program
https://github.com/everlastingsong/dev-null-program

This program stub Metaplex Token Metadata Program like /dev/null in Linux.
