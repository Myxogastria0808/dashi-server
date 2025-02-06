#!/bin/bash

# ping
/app/target/release/ping
# healthcheck and optional initialization
/app/target/release/healthcheck
# start dashi-server
/app/target/release/presentation
