#!/bin/bash
set -euxo pipefail
cd "$(dirname "$0")"

yarn install && yarn build