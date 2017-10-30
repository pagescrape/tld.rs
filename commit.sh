#!/bin/sh
git add .
message=`curl https://data.iana.org/TLD/tlds-alpha-by-domain.txt | head -n 1 | sed 's/# //'`
git commit -m "$message" && git push && cargo publish
