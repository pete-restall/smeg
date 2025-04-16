#!/bin/bash
set -e;
THISDIR="$(dirname "$(readlink -f "$0")")";
pushd ${THISDIR};
echo "Working directory: $(pwd)";

build_profile=dev; # TODO: we want a command-line argument for this at some point
if [ "x${build_profile}" == "xdev" ]; then
    build_profile_target=debug;
else
    build_profile_target=release;
fi;

board_manufacturer="$(echo "$1" | cut -d- -f 1)";
board_name="$(echo "$1" | cut -d- -f 2)";
board_configuration="$(echo "$1" | cut -d- -f 3)";
board_configuration=${board_configuration:-default}
board_triplet="${board_manufacturer}-${board_name}-${board_configuration}";

if [ $# -ne 1 ] || [ "x${board_manufacturer}" == "x" ] || [ "x${board_name}" == "x" ]; then
    echo "Usage: $0 <manufacturer>-<board>[-configuration]";
    echo "Example: $0 st-nucleo_l432kc-default";
    exit 1;
fi

echo "Building for:" \
    "profile=${build_profile} (${build_profile_target})," \
    "manufacturer=${board_manufacturer}," \
    "board=${board_name}," \
    "configuration=${board_configuration}";

case ${board_triplet} in
    ( st-nucleo_l432kc-* ) mcu_manufacturer=st; mcu_name=stm32l432kc ;;
esac;

if [ "x${mcu_manufacturer}" == "x" ]; then
    echo "Unknown board triplet '${board_triplet}'.";
    exit 2;
fi

mcu_dir=crates/mcus/${mcu_manufacturer}/${mcu_name};
target_dir=target/${board_triplet}/${build_profile_target};
target_path_prefix=${target_dir}/${board_triplet};
mkdir -p ${target_dir};

source_linker_script=${mcu_dir}/linker-script.lld;
target_linker_script=${target_path_prefix}.lld;
cp ${source_linker_script} ${target_linker_script};

target_json=${target_path_prefix}.json;
cp ${mcu_dir}/rust-target.json ${target_json};

cp ${mcu_dir}/openocd.cfg ${target_dir}/;

RUSTUP_TOOLCHAIN=nightly;
cargo +${RUSTUP_TOOLCHAIN} build -v \
    --profile ${build_profile} \
    --target ${target_json} \
    --features smeg-board-${board_manufacturer}-${board_name}-${board_configuration} \
    --config env.SMEG_SOURCE_LINKER_SCRIPT=\"${source_linker_script}\" \
    --config env.SMEG_TARGET_LINKER_SCRIPT=\"${target_linker_script}\" || exit 3;

flash_sections="-j .text -j .text.* -j .data -j .data.*";
image_prefix="${target_dir}/smeg-os";
echo;

echo "Generating ${image_prefix}.hex from ${image_prefix}.elf...";
rust-objcopy ${flash_sections} -O ihex ${image_prefix}.elf ${image_prefix}.hex || exit 4;

echo "Generating ${image_prefix}.bin from ${image_prefix}.elf...";
rust-objcopy ${flash_sections} -O binary ${image_prefix}.elf ${image_prefix}.bin || exit 5;
