#!/bin/sh

curl -u root:root -H "Accept:application/json" -H "NS:test" -H "DB:test" -X POST http://localhost:8000/sql --data-binary @info.sql
