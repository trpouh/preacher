#!/bin/zsh
cargo run -- \
    -r https://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:x-oauth-basic@github.com/trpouh/test-playbook.git \
    -b main \
    --tmp-dir tmp \
    -t examples \
    -s sermon.yaml