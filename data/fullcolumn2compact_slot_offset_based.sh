#!/bin/bash

# fullcolumn2compact.sh whirlpool-snapshot-x.csv.gz
FULL=$1

gunzip -c $FULL | cut -d, -f1,9 | gzip -c > $FULL.compact.csv.gz
