#!/bin/bash
mkdir -p demo_files
cd demo_files
for i in {1..2000000}
do
   touch "file${i}.txt"
done
