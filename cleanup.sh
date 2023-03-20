#!/bin/bash

rm -rf $(find rust/ -name target)
rm -rf $(find c*/ -name build)

for entry in $(find haskell -name Makefile); do
    make clean -C $(dirname $entry)
done
