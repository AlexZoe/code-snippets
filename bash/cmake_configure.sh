#!/bin/bash

SCRIPT_DIR="$(dirname "$(realpath "$0")")"
WORK_DIR=$(pwd)

OUTPUT_DIR=build
PRINT_HELP=
BUILD_MODE=Release

print_help() {
    echo "USAGE: .cmake_configure.sh [OPTION]"
    echo ""
    echo "OPTIONS"
    echo "  -h, --help      display this message and exit"
    echo "  -o, --out       choose build directory"
    echo "  -d, --debug     build in debug mode (default: release)"
    echo ""
}

while [[ $# -gt 0 ]]; do
    key="$1"

    case $key in
        -d|--debug)
            BUILD_MODE=Debug
            shift
            ;;
        -o|--out)
            OUTPUT_DIR="$2"
            shift
            shift
            ;;
        -h|--help)
            PRINT_HELP=1
            shift
            ;;
        *)    # unknown option
            shift
            ;;
    esac
done

if [[ $PRINT_HELP ]]; then
    print_help
    exit 0
fi

if [[ "$OUTPUT_DIR" != /* ]]; then
    $OUTPUT_DIR="$WORK_DIR/$OUTPUT_DIR"
fi

mkdir -p $OUTPUT_DIR && cd $OUTPUT_DIR

if [[ $? != 0 ]]; then
    echo "Cannot access $OUTPUT_DIR; check permissions"
    exit 1
fi

cmake $SCRIPT_DIR -DCMAKE_BUILD_TYPE=$BUILD_MODE && make -j

