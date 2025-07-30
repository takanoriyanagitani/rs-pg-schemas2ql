#!/bin/sh

export DATABASE_URL="postgresql://postgres@localhost:5433/postgres"
export LISTEN_ADDR="0.0.0.0:7298"

./schemas2ql
