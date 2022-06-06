#!/bin/bash

set -euo pipefail

export CONDUCTOR_CHAIN=2
export CONDUCTOR_ADDRESS="0000000000000000000000005c49f34d92316a2ac68d10a1e2168e16610e84f9"
export GLOBAL_KYC_AUTHORITY="1df62f291b2e969fb0849d99d9ce41e2f137006e"

cargo test -- --nocapture

BROWSER=""
anchor test
