#!/bin/bash

FILE=${1:-info}


curl -u root:root -H "Accept:application/json" -H "NS:test" -H "DB:test" -X POST http://localhost:8000/sql --data-binary @ql/${FILE}.ql
