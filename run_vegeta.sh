#!/bin/bash
echo "GET http://127.0.0.1:8000" | ./vegeta attack -rate="$1/s" -duration=1s | ./vegeta report

