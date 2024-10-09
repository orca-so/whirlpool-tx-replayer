use super::{assert_account_closed, assert_account_initialized, create_engine, ix, replay};

#[test]
fn test_open_and_close_position_with_token_extensions() {
    let mut engine = create_engine();

    let initialize_config = ix(
        "initializeConfig",
        r#"{"dataDefaultProtocolFeeRate": 300, "dataFeeAuthority": "6oCnhnbUz4WFsoC6P3SCmCPWFGmwQrSwvXXMdSoFekby", "dataCollectProtocolFeesAuthority": "5RxQK8kHgaVb6rHPvfTV9N8fciat8RQ4AsHDko1cSaXV", "dataRewardEmissionsSuperAuthority": "5GkuHxpU9bZvbSnXbFzizwxCkHieAkDu7mtmMpYTQ3xB", "keyWhirlpoolsConfig": "E2B7cgVcMxvNBX6eDHMSMXQeP5K3vFEEikGZTZZMEH2e", "keyFunder": "r21Gamwd9DtyjHeGywsneoQYR39C1VDwrw7tWxHAwh6", "keySystemProgram": "11111111111111111111111111111111"}"#,
    );
    let initialize_fee_tier = ix(
        "initializeFeeTier",
        r#"{"dataTickSpacing": 128, "dataDefaultFeeRate": 3000, "keyWhirlpoolsConfig": "E2B7cgVcMxvNBX6eDHMSMXQeP5K3vFEEikGZTZZMEH2e", "keyFeeTier": "5BkHdiwFBtwPZPiiJBByTV9pzB92gjprA3WDVaVYhWaK", "keyFunder": "r21Gamwd9DtyjHeGywsneoQYR39C1VDwrw7tWxHAwh6", "keyFeeAuthority": "6oCnhnbUz4WFsoC6P3SCmCPWFGmwQrSwvXXMdSoFekby", "keySystemProgram": "11111111111111111111111111111111"}"#,
    );
    let initialize_pool = ix(
        "initializePool",
        r#"{"dataTickSpacing": 128, "dataInitialSqrtPrice": "92233720368547758080", "keyWhirlpoolsConfig": "E2B7cgVcMxvNBX6eDHMSMXQeP5K3vFEEikGZTZZMEH2e", "keyTokenMintA": "25XQapbNkTHPTKHnGTUpkMXsAx5d2eH8BzLWy7z2PdvS", "keyTokenMintB": "5z8RPeq5yJHK3jzr3ykApoKTJPtxHi1AjREJu6uiTrhL", "keyFunder": "r21Gamwd9DtyjHeGywsneoQYR39C1VDwrw7tWxHAwh6", "keyWhirlpool": "DogkJidiZksWrZucykCu8DbNF3rwpbKArgthWkHZQVXY", "keyTokenVaultA": "CLaPCjnpQJFZV415MPWidLW1H53geFt5nZtHw5QT7XRY", "keyTokenVaultB": "B7BaQcSVdnpWQw1HoNrQ5RPtVPWVNkqzJrtvKamJRMuq", "keyFeeTier": "5BkHdiwFBtwPZPiiJBByTV9pzB92gjprA3WDVaVYhWaK", "keyTokenProgram": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA", "keySystemProgram": "11111111111111111111111111111111", "keyRent": "SysvarRent111111111111111111111111111111111", "decimalsTokenMintA": 0, "decimalsTokenMintB": 0}"#,
    );

    let open_position_with_token_extensions = ix(
        "openPositionWithTokenExtensions",
        r#"{"dataTickLowerIndex": 0, "dataTickUpperIndex": 128, "dataWithTokenMetadataExtension": 1, "keyFunder": "r21Gamwd9DtyjHeGywsneoQYR39C1VDwrw7tWxHAwh6", "keyOwner": "r21Gamwd9DtyjHeGywsneoQYR39C1VDwrw7tWxHAwh6", "keyPosition": "BbEMeYPTstMDgmohucEBj7H6obkinZQRcxZ2Gpt3cz3X", "keyPositionMint": "Hw3afBx59tPLCwVmE5rt6KpqWVGd8dfqKzSndKtuxHxa", "keyPositionTokenAccount": "EyExmEKtA9E45TKoBKjNRLyxuS2Bn5NsBrxZQ2fKrLE1", "keyWhirlpool": "DogkJidiZksWrZucykCu8DbNF3rwpbKArgthWkHZQVXY", "keyToken2022Program": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb", "keySystemProgram": "11111111111111111111111111111111", "keyAssociatedTokenProgram": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL", "keyMetadataUpdateAuth": "3axbTs2z5GBy6usVbNVoqEgZMng3vZvMnAoX29BFfwhr"}"#,
    );
    let close_position_with_token_extensions = ix(
        "closePositionWithTokenExtensions",
        r#"{"keyPositionAuthority": "r21Gamwd9DtyjHeGywsneoQYR39C1VDwrw7tWxHAwh6", "keyReceiver": "vvcvRBSqzAGjTKaPV3hECaGNbw94gLcoWFFpbvFHyP9", "keyPosition": "BbEMeYPTstMDgmohucEBj7H6obkinZQRcxZ2Gpt3cz3X", "keyPositionMint": "Hw3afBx59tPLCwVmE5rt6KpqWVGd8dfqKzSndKtuxHxa", "keyPositionTokenAccount": "EyExmEKtA9E45TKoBKjNRLyxuS2Bn5NsBrxZQ2fKrLE1", "keyToken2022Program": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"}"#,
    );

    let position = "BbEMeYPTstMDgmohucEBj7H6obkinZQRcxZ2Gpt3cz3X";

    replay(&mut engine, &initialize_config);
    replay(&mut engine, &initialize_fee_tier);
    replay(&mut engine, &initialize_pool);

    // open & close
    replay(&mut engine, &open_position_with_token_extensions);
    assert_account_initialized(&engine, position);
    replay(&mut engine, &close_position_with_token_extensions);
    assert_account_closed(&engine, position);

    // open & close (again)
    replay(&mut engine, &open_position_with_token_extensions);
    assert_account_initialized(&engine, position);
    replay(&mut engine, &close_position_with_token_extensions);
    assert_account_closed(&engine, position);
}
