#!/usr/bin/env bash

startup() {
    cargo test
    cargo build
}

test() {
    wanted="$1"
    jisp_content="$2"
    printf "%s" "$jisp_content" > /tmp/e2e_test.jisp
    result=$(./target/debug/jisp /tmp/e2e_test.jisp)
    if [ "$result" = "$wanted" ]; then
        echo "$jisp_content => $result"
    else
        echo "$jisp_content => $wanted expected, got $result"
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
test 15 '(+ 1 2 3 4 5)'
test 15 '(+ (+ 1 2) (+ 3 4 5))'

test 4 '(+ 1 1 (- 4 2))'
test 5 '(+ 5 (- 5 2 3) (- 13 13))'

test 20 '(* 4 5)'
test 3 '(/ 12 4)'
test 23 '(+ (/ 12 4) (* 4 5))'

test true '(== (/ 12 4) (+ 1 2))'
test false '(!= (/ 12 4) (+ 1 2))'
test true '(< 1 3)'
test true '(<= 1 3)'
test false '(<= 4 3)'
test true '(> 4 3)'
test true '(>= 4 3)'
test false '(> 3 3)'
test true '(>= 3 3)'

cleanup