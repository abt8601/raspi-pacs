#!/bin/sh -e

print_usage() {
    echo 'Usage: gen.sh MCU_NAME' >&2
}

if [ $# -ne 1 ]; then
    print_usage
    exit 1
fi

mcu_name=$1

cd "crates/${mcu_name}-lpa/"

svd2rust -i "../../peripherals/svd/gen/${mcu_name}_lpa.svd" --target none \
    --atomics --impl_debug

rm -rf src/
# TODO: Switch back to using `form` as soon as djmcgill/form#12 is resolved.
mkdir src/
mv lib.rs src/
cargo fmt
