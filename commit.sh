#!/bin/sh

export OVERCOMMIT_DISABLE=1

./generate
cargo bump minor

git add .
message=`curl https://data.iana.org/TLD/tlds-alpha-by-domain.txt | head -n 1 | sed 's/# //'`
git commit -m "$message" && git push && cargo publish
