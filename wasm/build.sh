#!/bin/bash
set -euxo pipefail
cd "$(dirname "$0")"

VUE_OUTPUT_PATH='../vue/cubetrain/src/wasm'
NGINX_OUTPUT_PATH='../nginx/wasm'

cargo make build_release && \
echo '/* eslint-disable */' > ${VUE_OUTPUT_PATH}/package.js && \
echo 'if (typeof global.TextEncoder === "undefined") { global.TextEncoder = require("util").TextEncoder }' >> ${VUE_OUTPUT_PATH}/package.js && \
echo 'if (typeof global.TextDecoder === "undefined") { global.TextDecoder = require("util").TextDecoder }' >> ${VUE_OUTPUT_PATH}/package.js && \
echo 'if (typeof global.fetch === "undefined") { const fetchPolifill = require("whatwg-fetch"); global.fetch = fetchPolifill.fetch;}' >> ${VUE_OUTPUT_PATH}/package.js && \
cat pkg/package.js >> ${VUE_OUTPUT_PATH}/package.js && \
cp pkg/package_bg.wasm ${VUE_OUTPUT_PATH}/package_bg.wasm && \
cp pkg/package.js ${NGINX_OUTPUT_PATH}/package.js && \
cp pkg/package_bg.wasm ${NGINX_OUTPUT_PATH}/package_bg.wasm