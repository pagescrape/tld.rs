#!/bin/sh
git add .
message=`curl https://data.iana.org/TLD/tlds-alpha-by-domain.txt | head -n 1`
git commit -m "$message"
