#!/usr/bin/env bash

set -e

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

function get_md5sum() {
    printf "$(md5sum $1 | awk '{print $1}')"
}

# Check if python is installed.
if ! command -v python3 2>&1 >/dev/null; then
    echo "Python 3 is required.  Please install the Python 3."
    exit 1
fi
if ! command -v md5sum 2>&1 >/dev/null; then
    echo "md5sum is required.  Please install the md5sum."
    exit 1
fi

# Create the venv if not exist.
if [[ ! -d $SCRIPT_DIR/venv ]]; then
    python3 -m venv $SCRIPT_DIR/venv
fi

# Activate the venv.
source $SCRIPT_DIR/venv/bin/activate

# Install the requirements.
if [[ ! -f $SCRIPT_DIR/venv/requirements.md5sum ]]; then
    python3 -m pip install -q -r $SCRIPT_DIR/requirements.txt >/dev/null 2>&1 
    get_md5sum $SCRIPT_DIR/requirements.txt > $SCRIPT_DIR/venv/requirements.md5sum
fi
wanted_md5sum="$(get_md5sum $SCRIPT_DIR/requirements.txt)"
if [[ $wanted_md5sum != "$(cat $SCRIPT_DIR/venv/requirements.md5sum)" ]]; then
    python3 -m pip install -q -r $SCRIPT_DIR/requirements.txt >/dev/null 2>&1
    get_md5sum $SCRIPT_DIR/requirements.txt > $SCRIPT_DIR/venv/requirements.md5sum
fi

# Run.
python3 tests/main.py