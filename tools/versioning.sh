#!/usr/bin/env sh

if [ -z "$1" ]; then
    echo "No version info supplied"
    exit 1
fi

yarn ts-node tools/version.ts "$1"
