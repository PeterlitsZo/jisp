#!/usr/bin/env bash

startup() {
    cargo build --release
}

test() {
    wanted="$1"
    jisp_content="$2"
    printf "%s" "$jisp_content" > /tmp/e2e_test.jisp
    result=$(./target/release/jisp /tmp/e2e_test.jisp)
    if [ "$result" = "$wanted" ]; then
        echo "$jisp_content => $result"
    else
        echo "$jisp_content => $wanted expected"
        exit 1
    fi
}

cleanup() {
    rm /tmp/e2e_test.jisp
}

startup

test 0 '0'
test 1 '1'
test 2 '2'

test 2 '(+ 1 1)'
test 3 '(+ 1 2)'
test 350 '(+ 100 250)'

cleanup