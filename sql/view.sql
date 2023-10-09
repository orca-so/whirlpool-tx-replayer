DROP VIEW vwixsSwap;
CREATE VIEW vwixsSwap AS
SELECT
    t.txid,
    t.order,
    (SELECT signature FROM txs WHERE txid = t.txid) AS "signature",
    "swap" AS "ix",
    JSON_OBJECT(
        'dataAmount', t.dataAmount,
    	'dataOtherAmountThreshold', t.dataOtherAmountThreshold,
		'dataSqrtPriceLimit', t.dataSqrtPriceLimit,
		'dataAmountSpecifiedIsInput', t.dataAmountSpecifiedIsInput,
		'dataAToB', t.dataAToB,
		'keyTokenProgram', toPubkeyBase58(t.keyTokenProgram),
		'keyTokenAuthority', toPubkeyBase58(t.keyTokenAuthority),
		'keyWhirlpool', toPubkeyBase58(t.keyWhirlpool),
		'keyTokenOwnerAccountA', toPubkeyBase58(t.keyTokenOwnerAccountA),
		'keyVaultA', toPubkeyBase58(t.keyVaultA),
		'keyTokenOwnerAccountB', toPubkeyBase58(t.keyTokenOwnerAccountB),
		'keyVaultB', toPubkeyBase58(t.keyVaultB),
		'keyTickArray0', toPubkeyBase58(t.keyTickArray0),
		'keyTickArray1', toPubkeyBase58(t.keyTickArray1),
		'keyTickArray2', toPubkeyBase58(t.keyTickArray2),
		'keyOracle', toPubkeyBase58(t.keyOracle),
		'transferAmount0', t.transferAmount0,
		'transferAmount1', t.transferAmount1
	) AS "json"
FROM ixsSwap t;

CREATE VIEW vwixsAll AS
SELECT * FROM vwixsSwap;
