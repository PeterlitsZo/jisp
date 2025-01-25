#!/usr/bin/env bash

# Print the usage of the script and exit by given argument.
usage() {
    echo "Usage: $0 <test|build-cov|show>"
    echo
    echo '  test       Test, and generate the coverage report.'
    echo '  build-cov  Generate the HTML coverage report.'
    echo '  show       Serve the HTML coverage report.'
    echo
    exit $1
}

if (( $# != 1 )); then
    usage 1
fi

if [[ "$1" == "test" ]]; then
    # Run the `cargo test` and generate the coverage report as `cargo-test.profraw`.

    rm *.profraw

    export CARGO_INCREMENTAL=0
    export RUSTFLAGS='-Cinstrument-coverage'
    export LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw'

    cargo test
elif [[ "$1" == "build-cov" ]]; then
    # Generate the HTML coverage report from the `cargo-test.profraw` file.

    rm -rf target/coverage
    if ! command -v grcov &> /dev/null; then
        echo 'The grcov is not installed. Please install it by `cargo install grcov`.'
        echo
        echo 'More info: https://github.com/mozilla/grcov'
        exit 1
    fi
    grcov . --binary-path ./target/debug/deps/ \
        -s . -t html --branch \
        --ignore-not-existing --ignore '../*' --ignore "/*" \
        -o target/coverage/html
elif [[ "$1" == "show" ]]; then
    # Serve the HTML coverage report by Python and love.

    cd target/coverage/html
    python3 -m http.server
elif [[ "$1" == "help" ]]; then
    usage 0
else
    usage 1
fi