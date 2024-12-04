# whirlpool-replayer

## Indexed Historical Data
https://whirlpool-archive.pleiades.dev/alpha/index.html

## Motivation & Core Concept
Replaying the entire Solana would require a lot of resources, but for Whirlpool state alone, replay is possible with a snapshot of the account and all successful transactions associated with Whirlpool program.

Replay realize easy reproduction of the state at all points in time in the past.

``whirlpool-replayer`` library does not require a large amount of resources and is intended to be able to run even on a simple laptop.

This library will give developers free access to Whirlpool history since its genesis until yesterday.

![key_concept](https://github.com/orca-so/whirlpool-tx-replayer/assets/109891005/4eaf6b2b-e40b-4d74-8a0c-500348ae13e9)

## Implementation Architecture
- ``replay-engine``
  - manage slot state (slot, blockHeight, blockTime)
  - manage account state
  - manage program data state
  - execute instruction with bank
- ``whirlpool-replayer``
  - fetch required state and transaction from data storage
  - initialize replay-engine
  - execute replaying with callback

![architecture](https://github.com/orca-so/whirlpool-tx-replayer/assets/109891005/528286f0-82d5-43c3-9e13-d1a51152e63f)

### Data storage
- Data is clearly divided into states and transactions.
- Data is treated as one unit for one day based on blockTime.
- Whirlpool Program data is also included in the states to handle program upgrade.
- The relation between states and transactions is as follows:

![key_formula](https://github.com/orca-so/whirlpool-tx-replayer/assets/109891005/60a98f7d-36ab-427e-95c8-35f82e3caeed)

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
- When memory is scarce, using RocksDB to store accounts can limit memory usage to a few hundred megabytes.

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
- With ``-m``, we store all accounts data in memory (faster).
```
$ cargo run --release -p whirlpool-replay -- -m data/sample_local_storage 20220407

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

## Public Remote Storage Endpoint

- https://whirlpool-archive.pleiades.dev/alpha
- (deprecated) https://whirlpool-replay.pleiades.dev/alpha

### Browser access
https://whirlpool-archive.pleiades.dev/alpha/index.html

### Program access
https://whirlpool-archive.pleiades.dev/alpha/index.json

### Manual download of state and transaction files

If you will use ``whirlpool-replayer`` heavily, to avoid duplicated data download, please download files and put them locally.

*Path*
- state: ``<REMOTE STORAGE ENDPOINT>``/``<YYYY>``/``<MMDD>``/whirlpool-state-``<YYYYMMDD>``.json.gz
- transaction: ``<REMOTE STORAGE ENDPOINT>``/``<YYYY>``/``<MMDD>``/whirlpool-transaction-``<YYYYMMDD>``.jsonl.gz

*Range*
- From: The genesis of Whirlpool Program (20220309)
- To: Yesterday (TODAY is not covered)

*Example*
```
curl -OL https://whirlpool-archive.pleiades.dev/alpha/2023/1130/whirlpool-state-20231130.json.gz
```
```
curl -OL https://whirlpool-archive.pleiades.dev/alpha/2023/1130/whirlpool-transaction-20231130.jsonl.gz
```

## Whirlpool Event Stream
Whirlpool Event Stream (Whirlpool Now) is live. (EXPERIMENTAL)

https://github.com/yugure-orca/whirlpool-now-doc/blob/main/README.md

Using event stream, you can track & detect EVERYTHING on Whirlpool program without decoding and replay difficulty:

- track trades (including tickIndex and sqrtPrice)
- track pool price
- track liquidity operations (deposit & withdraw)
- detect new whirlpool
- detect new reward emission

The beauty of this is that you can start at any point in the past three days.
In the event of a disconnection, it can be resumed without loss of data.

Of course Whirlpool Now is baked by `whirlpool-replayer` library.
So you even have access to some important account fields that is not directly recorded in the block.

## TODO
### Replace GZip by ZStandard
- We can reduce >30% storage
- It is so fast
- works well even in streaming mode

### Add test cases for each instruction replay handler

### Error handling
- eliminate `unwrap` (panic)
- eliminate `anyhow`

### More performance tuning
- ~~ConfirmedTransactionWithStatusMeta::encode in execute_transaction is the next hotspot~~
- ~~Delete unused features such as token balance snapshots~~
- 1.17.22 may be slower than 1.16.18
- find next hot spot

### Validation at instruction level
By performing the following verification before and after each instruction is executed, abnormal situations may be detected at an early stage.

- Accounts that should not exist do not exist
- Accounts that should exist do exist
- Token volume consistent with the transaction log has been transferred

### More callback
- ~~slot_begin_callback~~ (implemented as slot_pre_callback)
- ~~slot_end_callback~~ (implemented as slot_post_callback)
- transaction_begin_callback (no need for now)
- transaction_end_callback (no need for now)

### WASM build for client use / direct use from Node.js (Typescript) similar to Bankrun
https://kevinheavey.github.io/solana-bankrun/

For now, I think it is NOT needed to support other language.
We should support them by providing data stream.

### ~~Handling new deployment release~~
We know when whirlpool program have been updated in the past exactly.
But we cannot know which slot and which tx index in the block the next version will be released.

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

### ~~Using disk to hold state (reduce memory usage)~~
- introduce AccountDataStore
- AccountDataStore on memory
- AccountDataStore on disk (RocksDB)

### ~~Fnmut callback, async callback~~
Using Fn, but we can collect info via Rc & RefCell.

### ~~Support V2 instructions (TokenExtensions)~~

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
