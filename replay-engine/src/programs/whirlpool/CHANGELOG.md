## 2026
### 20261015-457147740
- Remove migrate_repurpose_reward_authority_space ix ([PR1311](https://github.com/orca-so/whirlpools/pull/1311))
- Fix: u256 div edge case ([PR1326](https://github.com/orca-so/whirlpools/pull/1326))
- Prepare for SIMD-0460 Virtual Address Space Adjustments ([PR1392](https://github.com/orca-so/whirlpools/pull/1392))
- Improve checkpoint update (Mitigation for griefing attacks) ([PR1396](https://github.com/orca-so/whirlpools/pull/1396))

### 20260819-440170207
- Prepare for rent reduction starting with Agave 4.2 ([PR1366](https://github.com/orca-so/whirlpools/pull/1366))

### 20260204-398059635
- Pinocchio implementation for Liquidity ops ([PR1226](https://github.com/orca-so/whirlpools/pull/1226))
- Add reposition_liquidity_v2 instruction ([PR1227](https://github.com/orca-so/whirlpools/pull/1227))
- Add increase_liquidity_by_token_amounts_v2 instruction ([PR1229](https://github.com/orca-so/whirlpools/pull/1229))

### 20260122-395191896
- Add open one-sided position logic ([PR1189](https://github.com/orca-so/whirlpools/pull/1189))
- Add set_adaptive_fee_constants instruction ([PR1187](https://github.com/orca-so/whirlpools/pull/1187))
- Update metadata URL for dynamic position NFTs ([PR1218](https://github.com/orca-so/whirlpools/pull/1218))

## 2025
### 20250911-366188902
- Make ADMINS multi-sig (no feature update) ([PR1061](https://github.com/orca-so/whirlpools/pull/1061))

### 20250821-361491090
- Non transferable position ([PR1038](https://github.com/orca-so/whirlpools/pull/1038))
- Safer account initialization ([PR1010](https://github.com/orca-so/whirlpools/pull/1010))
- Remove unnecessary transfer hook token program account info ([PR1004](https://github.com/orca-so/whirlpools/pull/1004))
- Refactor Dynamic TA ([PR1006](https://github.com/orca-so/whirlpools/pull/1006))

### 20250705-351188208
- Dynamic TickArray ([PR970](https://github.com/orca-so/whirlpools/pull/970))

### 20250625-349119692
- Support new TokenExtensions ([PR974](https://github.com/orca-so/whirlpools/pull/974))

### 20250610-345968990
- (Partial release)Dynamic TickArray ([PR973](https://github.com/orca-so/whirlpools/pull/973))

### 20250513-339751616
- Reduce stack size usage on initialize_pool_with_adaptive_fee ([PR946](https://github.com/orca-so/whirlpools/pull/946))

### 20250505-338005185
- Adaptive Fee ([PR918](https://github.com/orca-so/whirlpools/pull/918))

### 20250421-334826946
- Reset position range ([PR902](https://github.com/orca-so/whirlpools/pull/902))
- Allow locking concentrated positions ([PR903](https://github.com/orca-so/whirlpools/pull/903))
- Transfer Locked Position ([PR904](https://github.com/orca-so/whirlpools/pull/904))
- Fix event emission / transfer fee value on Traded event (swapV2) ([PR905](https://github.com/orca-so/whirlpools/pull/905))

### 20250228-323677107
- Event emission ([PR778](https://github.com/orca-so/whirlpools/pull/778))
- Liquidity Locking ([PR768](https://github.com/orca-so/whirlpools/pull/768))
- Increase MAX_FEE_RATE ([PR743](https://github.com/orca-so/whirlpools/pull/743))

## 2024
### 20241011-294954598
- TokenExtensions based Position NFT ([PR341](https://github.com/orca-so/whirlpools/pull/341))
- InterestBearing support ([PR294](https://github.com/orca-so/whirlpools/pull/294))

### 20241001-293034932
- Fix TransferHook additional account derive logic ([PR291](https://github.com/orca-so/whirlpools/pull/291))
- Update swap math for SplashPool edge-case ([PR244](https://github.com/orca-so/whirlpools/pull/244))

### 20240902-287352124
- Repr(C, packed) ([PR250](https://github.com/orca-so/whirlpools/pull/250))
- Reject Partial fill in ExactOut ([PR277](https://github.com/orca-so/whirlpools/pull/277))

### 20240813-283380456
- SparseSwap ([PR159](https://github.com/orca-so/whirlpools/pull/159))

### 20240720-278646699
- SplashPool ([PR161](https://github.com/orca-so/whirlpools/pull/161))

### 20240528-268396603
- TokenExtensions support ([PR134](https://github.com/orca-so/whirlpools/pull/134))

## 2023
### 20230901-214946722
- Increase the fee rate limit and log fee growth ([PR104](https://github.com/orca-so/whirlpools/pull/104))

### 20230419-189278833
- Position bundle and Anchor v26 ([PR89](https://github.com/orca-so/whirlpools/pull/89))

### 20230217-178191491
- Two hop swap instruction ([PR87](https://github.com/orca-so/whirlpools/pull/87))

### 20230127-174717190
- Update Whirlpool position NFT metadata URI ([PR83](https://github.com/orca-so/whirlpools/pull/83))

## 2022
### 20221221-168120167
- remove temporary adminIncreaseLiquidity instruction (PR82 related state patch) 

### 20221221-168116472
- add temporary adminIncreaseLiquidity instruction (PR82 related state patch)

### 20221204-164780893
- Handle liquidity swapping edge case across initialized tick boundary ([PR82](https://github.com/orca-so/whirlpools/pull/82))

### 20220503
Initial commit for the open-source repository
