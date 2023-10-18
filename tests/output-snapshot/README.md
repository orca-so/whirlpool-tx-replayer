## To compare 2 snapshots
A snapshot file is simple KV csv (pubkeyInBase58,dataInBase64) file, but not sorted by pubkey.

So we need to sort them before comparison.

## Steps
```
gzcat output-snapshot/whirlpool-snapshot-<slot>.csv.gz | sort > output.csv
```
```
gzcat target-snapshot/whirlpool-snapshot-<slot>.csv.gz | sort > target.csv
```
```
diff output.csv target.csv
```