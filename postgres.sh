#!/bin/bash

set -euo pipefail

docker container run --name async-rocket-postgres \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_DB=postgres \
  -p 5432:5432 \
  -v data:/var/lib/postgresql/data \
  -d postgres:latest
