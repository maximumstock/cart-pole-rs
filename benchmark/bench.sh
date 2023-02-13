#!/usr/bin/env bash

FILENAME="results-$(date +"%Y-%m-%dT%H:%M:%S%z").bin"
vegeta attack -duration 30s -rate 200 -name inference-local -targets targets.txt > $FILENAME
cat $FILENAME | vegeta report
cat $FILENAME | vegeta plot > plot.local.html

if [[ "$OSTYPE" == "darwin"* ]]; then
    open plot.local.html
fi