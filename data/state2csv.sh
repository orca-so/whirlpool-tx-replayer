#!/bin/bash

# state2csv.sh state.json.gz
STATE=$1

gunzip -c $STATE | jq -r '.accounts[] | [.pubkey, .data] | @csv' | tr -d '"' | sort > $STATE.csv

