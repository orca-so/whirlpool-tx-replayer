#!/bin/bash

# rewrite-program-data.sh state.json.gz program.so
# Note: this script used fixed temporary file names, so it can't be run in parallel.

STATE_GZ_FILE=$1
PROGRAM_SO_FILE=$2

TMP_PROGRAM_DATA_PART=program_data.part
TMP_STATE_DATA_PART=state_data.part

echo "Rewriting program data in $STATE_GZ_FILE with $PROGRAM_SO_FILE"

# create Base64 encoded program data
base64 $PROGRAM_SO_FILE | sed -e 's/^/"programData": "/' -e 's/$/"/' > $TMP_PROGRAM_DATA_PART
# cut "programData" and trailing "}" from the state file
gunzip -c $STATE_GZ_FILE | sed 's/"programData".*//' > $TMP_STATE_DATA_PART

# append the program data and "}" to the state file
cat $TMP_PROGRAM_DATA_PART >> $TMP_STATE_DATA_PART
echo "}" >> $TMP_STATE_DATA_PART

# overwrite the state file
cat $TMP_STATE_DATA_PART | jq -c | gzip -c > $STATE_GZ_FILE

# cleanup
rm $TMP_PROGRAM_DATA_PART $TMP_STATE_DATA_PART

echo "Done"
