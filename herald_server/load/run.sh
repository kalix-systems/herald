#!/usr/bin/env bash
wrk -t8 -c256 -d5s -s script.lua --latency http://localhost:8080/users_exist
