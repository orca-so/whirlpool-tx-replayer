use super::{assert_account_initialized, create_engine, ix, replay};

#[test]
fn test_transfer_locked_position() {
    let mut engine = create_engine();

    let initialize_config = ix(
        "initializeConfig",
        r#"{"dataDefaultProtocolFeeRate": 300, "dataFeeAuthority": "3Q4pMMf5e2wcqYMGZhjpDWEmWaAVAVzMxKJtsv2iMu1R", "dataCollectProtocolFeesAuthority": "EXz6jkK22aa6jqgkG4q3GXm5Ankb6gh8Dsk4VytWupf7", "dataRewardEmissionsSuperAuthority": "34H4AgZt4CpS4488bWNcXUZFqpbXrfCnS7ika2BUwY3p", "keyWhirlpoolsConfig": "Drks8NktGoBWtV5YPWwKEWR9skUWypjmS6q46ckjPi8D", "keyFunder": "r21Gamwd9DtyjHeGywsneoQYR39C1VDwrw7tWxHAwh6", "keySystemProgram": "11111111111111111111111111111111"}"#,
    );
    let initialize_fee_tier = ix(
        "initializeFeeTier",
        r#"{"dataTickSpacing": 64, "dataDefaultFeeRate": 3000, "keyWhirlpoolsConfig": "Drks8NktGoBWtV5YPWwKEWR9skUWypjmS6q46ckjPi8D", "keyFeeTier": "C7HVbbKnAnuXfhZ87mefqWPXDo2cjrj35yYmq3HhV1D6", "keyFunder": "r21Gamwd9DtyjHeGywsneoQYR39C1VDwrw7tWxHAwh6", "keyFeeAuthority": "3Q4pMMf5e2wcqYMGZhjpDWEmWaAVAVzMxKJtsv2iMu1R", "keySystemProgram": "11111111111111111111111111111111"}"#,
    );
    let initialize_pool_v2 = ix(
        "initializePoolV2",
        r#"{"dataTickSpacing": 64, "dataInitialSqrtPrice": "92233720368547758080", "keyWhirlpoolsConfig": "Drks8NktGoBWtV5YPWwKEWR9skUWypjmS6q46ckjPi8D", "keyTokenMintA": "7Xws5FruPQGB3Jq9xj4Cc55rruVjY8mdzmmr6wpUHeqB", "keyTokenMintB": "7j8yxRszXTonjCS7LsnvLtitRSBCggPM2Bx9yzJjcT9y", "keyTokenBadgeA": "FpegWLV3Gj8ye8gP9Jc5VPWXm6q4cWCSHoc6g56KYEyv", "keyTokenBadgeB": "AKC42rsBPMDu2F4DfSjzj3SeB2CGMEdbhVTDVJjrxM9N", "keyFunder": "r21Gamwd9DtyjHeGywsneoQYR39C1VDwrw7tWxHAwh6", "keyWhirlpool": "BsGwEuUqbfeUSDN4mmxhcGFhNYKypKH8NZjoQ7DQrFfC", "keyTokenVaultA": "FNiNQiXYgFhKcKuU16DuNDxZynVAmNG2DVs3ukXe1JeB", "keyTokenVaultB": "6tMEfTsiby8m1jh861Zb23aTQt65c8mjVvL6PgkjXmjh", "keyFeeTier": "C7HVbbKnAnuXfhZ87mefqWPXDo2cjrj35yYmq3HhV1D6", "keyTokenProgramA": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA", "keyTokenProgramB": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA", "keySystemProgram": "11111111111111111111111111111111", "keyRent": "SysvarRent111111111111111111111111111111111", "decimalsTokenMintA": 0, "decimalsTokenMintB": 0}"#,
    );
    let initialize_tick_array_neg_444928 = ix(
        "initializeTickArray",
        r#"{"dataStartTickIndex": -444928, "keyWhirlpool": "BsGwEuUqbfeUSDN4mmxhcGFhNYKypKH8NZjoQ7DQrFfC", "keyFunder": "r21Gamwd9DtyjHeGywsneoQYR39C1VDwrw7tWxHAwh6", "keyTickArray": "CPWekMYLLoEggpQCV4ddND6pGCo4LcGb13uvSmsBHfpc", "keySystemProgram": "11111111111111111111111111111111"}"#,
    );
    let initialize_tick_array_439296 = ix(
        "initializeTickArray",
        r#"{"dataStartTickIndex": 439296, "keyWhirlpool": "BsGwEuUqbfeUSDN4mmxhcGFhNYKypKH8NZjoQ7DQrFfC", "keyFunder": "r21Gamwd9DtyjHeGywsneoQYR39C1VDwrw7tWxHAwh6", "keyTickArray": "ESzF37B5Z3JzjU47sMAymWXfrbD2RoezWEnnPiATtvt2", "keySystemProgram": "11111111111111111111111111111111"}"#,
    );
    let open_position_with_token_extensions = ix(
        "openPositionWithTokenExtensions",
        r#"{"dataTickLowerIndex": -443584, "dataTickUpperIndex": 443584, "dataWithTokenMetadataExtension": 1, "keyFunder": "r21Gamwd9DtyjHeGywsneoQYR39C1VDwrw7tWxHAwh6", "keyOwner": "r21Gamwd9DtyjHeGywsneoQYR39C1VDwrw7tWxHAwh6", "keyPosition": "22MwAtBfaqJQxNH5kHrdZdaTERH9bdT5mqGBfSpdGV9b", "keyPositionMint": "E1EGF4YqwPa4uR2naSJ37n22XHaiqQ616NXv6fYLWpk1", "keyPositionTokenAccount": "CqybBwB821UWPgJuvERUZPUiRoMpBnsDELL7KBQEpKcJ", "keyWhirlpool": "BsGwEuUqbfeUSDN4mmxhcGFhNYKypKH8NZjoQ7DQrFfC", "keyToken2022Program": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb", "keySystemProgram": "11111111111111111111111111111111", "keyAssociatedTokenProgram": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL", "keyMetadataUpdateAuth": "3axbTs2z5GBy6usVbNVoqEgZMng3vZvMnAoX29BFfwhr"}"#,
    );
    let increase_liquidity = ix(
        "increaseLiquidity",
        r#"{"dataLiquidityAmount": "1000000", "dataTokenAmountMaxA": "200000", "dataTokenAmountMaxB": "5000000", "keyWhirlpool": "BsGwEuUqbfeUSDN4mmxhcGFhNYKypKH8NZjoQ7DQrFfC", "keyTokenProgram": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA", "keyPositionAuthority": "r21Gamwd9DtyjHeGywsneoQYR39C1VDwrw7tWxHAwh6", "keyPosition": "22MwAtBfaqJQxNH5kHrdZdaTERH9bdT5mqGBfSpdGV9b", "keyPositionTokenAccount": "CqybBwB821UWPgJuvERUZPUiRoMpBnsDELL7KBQEpKcJ", "keyTokenOwnerAccountA": "7RJCL297iWxQGNiEvdLW8srWE2HFqH4WrQXiHMnXD18", "keyTokenOwnerAccountB": "CPGfEURMHiLjvsjAC45XesbqVAfDQDbutK4HmiMLGLTH", "keyTokenVaultA": "FNiNQiXYgFhKcKuU16DuNDxZynVAmNG2DVs3ukXe1JeB", "keyTokenVaultB": "6tMEfTsiby8m1jh861Zb23aTQt65c8mjVvL6PgkjXmjh", "keyTickArrayLower": "CPWekMYLLoEggpQCV4ddND6pGCo4LcGb13uvSmsBHfpc", "keyTickArrayUpper": "ESzF37B5Z3JzjU47sMAymWXfrbD2RoezWEnnPiATtvt2", "transferAmount0": "200000", "transferAmount1": "5000000"}"#,
    );

    let lock_position = ix(
        "lockPosition",
        r#"{"dataLockType": {"name":"permanent"}, "keyFunder": "r21Gamwd9DtyjHeGywsneoQYR39C1VDwrw7tWxHAwh6", "keyPositionAuthority": "r21Gamwd9DtyjHeGywsneoQYR39C1VDwrw7tWxHAwh6", "keyPosition": "22MwAtBfaqJQxNH5kHrdZdaTERH9bdT5mqGBfSpdGV9b", "keyPositionMint": "E1EGF4YqwPa4uR2naSJ37n22XHaiqQ616NXv6fYLWpk1", "keyPositionTokenAccount": "CqybBwB821UWPgJuvERUZPUiRoMpBnsDELL7KBQEpKcJ", "keyLockConfig": "66HeXxFvnaBEvi5YBjHxBsmguiTustHA9hWQPBjSrtwX", "keyWhirlpool": "BsGwEuUqbfeUSDN4mmxhcGFhNYKypKH8NZjoQ7DQrFfC", "keyToken2022Program": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb", "keySystemProgram": "11111111111111111111111111111111"}"#,
    );

    let transfer_locked_position = ix(
        "transferLockedPosition",
        r#"{"keyPositionAuthority": "r21Gamwd9DtyjHeGywsneoQYR39C1VDwrw7tWxHAwh6", "keyReceiver": "r21Gamwd9DtyjHeGywsneoQYR39C1VDwrw7tWxHAwh6", "keyPosition": "22MwAtBfaqJQxNH5kHrdZdaTERH9bdT5mqGBfSpdGV9b", "keyPositionMint": "E1EGF4YqwPa4uR2naSJ37n22XHaiqQ616NXv6fYLWpk1", "keyPositionTokenAccount": "CqybBwB821UWPgJuvERUZPUiRoMpBnsDELL7KBQEpKcJ", "keyDestinationTokenAccount": "2J49LG3g1r2QW3N3L2CS2NKTgJ7gV86kuke3YqGBzshU", "keyLockConfig": "66HeXxFvnaBEvi5YBjHxBsmguiTustHA9hWQPBjSrtwX", "keyToken2022Program": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"}"#,
    );

    let lock_config = "66HeXxFvnaBEvi5YBjHxBsmguiTustHA9hWQPBjSrtwX";

    replay(&mut engine, &initialize_config);
    replay(&mut engine, &initialize_fee_tier);
    replay(&mut engine, &initialize_pool_v2);
    replay(&mut engine, &initialize_tick_array_neg_444928);
    replay(&mut engine, &initialize_tick_array_439296);
    replay(&mut engine, &open_position_with_token_extensions);
    replay(&mut engine, &increase_liquidity);
    replay(&mut engine, &lock_position);
    replay(&mut engine, &transfer_locked_position);

    //TODO: fix
    //FIXME: check the new owner (but it is not impossible)
    assert_account_initialized(&engine, lock_config);
}
