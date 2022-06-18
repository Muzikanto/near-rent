#!/bin/bash
source ../scripts/neardev/dev-account.env
near call $CONTRACT_NAME test --accountId $CONTRACT_NAME "{ \"token_id\": \"8388\" }"
