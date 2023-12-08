#!/bin/bash

# usage: snapshot2state.sh <slot> <blockHeight> <blockTime> <snapshot.csv.gz> <program.so>

SLOT=$1
BLOCK_HEIGHT=$2
BLOCK_TIME=$3
SNAPSHOT_FILE=$4
PROGRAM_FILE=$5

echo slot: $SLOT
echo block-height: $BLOCK_HEIGHT
echo block-time: $BLOCK_TIME
echo snapshot: $SNAPSHOT_FILE
echo program: $PROGRAM_FILE

WORKFILE=work.$SLOT.tmp
RESULTFILE=whirlpool-state-$SLOT.json.gz

echo '{'                                     > $WORKFILE
echo   '"slot":' $SLOT ','                  >> $WORKFILE
echo   '"blockHeight":' $BLOCK_HEIGHT ','   >> $WORKFILE
echo   '"blockTime":' $BLOCK_TIME ','       >> $WORKFILE
echo   '"accounts": ['                      >> $WORKFILE

# 8if9...m8GE & 2KFq...rcyn are IDL related accounts
gunzip -c $SNAPSHOT_FILE | \
   egrep -v '^8if9aDeshh3iGLasCCzBGQyxAvU8Q4jUZGF3N5sVm8GE,' | \
   egrep -v '^2KFqE4RWoPVbvodo8vbggCFeHPS8TDvgpwp79ALMrcyn,' | \
   awk -F, '{ print "{ \"pubkey\":\"" $1 "\", \"data\":\"" $2 "\" }," }' | \
   tr -d '\n' | \
   sed 's/,$//' >> $WORKFILE

echo   '],'                                 >> $WORKFILE
echo   '"programData": "'                   >> $WORKFILE

base64 $PROGRAM_FILE                        >> $WORKFILE

echo   '"'                                  >> $WORKFILE
echo '}'                                    >> $WORKFILE

# remove space and newline
cat $WORKFILE | tr -d ' \n' | gzip -c > $RESULTFILE
