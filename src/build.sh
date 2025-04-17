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

has_std=false;
has_target_json=true;
has_linker_script=true;
has_flash_image=true;
link_with_libc=false;

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
    ( host-native-* )
        mcu_manufacturer=host;
        mcu_name=native;
        has_target_json=false;
        has_linker_script=false;
        has_flash_image=false;
        link_with_libc=true;
        ;;

    ( st-nucleo_l432kc-* ) mcu_manufacturer=st; mcu_name=stm32l432kc ;;
esac;

if [ "x${mcu_manufacturer}" == "x" ]; then
    echo "Unknown board triplet '${board_triplet}'.";
    exit 2;
fi

mcu_dir=crates/mcus/${mcu_manufacturer}/${mcu_name};
target_triplet_dir=target/${board_triplet};
target_dir=${target_triplet_dir}/${build_profile_target};
target_path_prefix=${target_dir}/${board_triplet};
mkdir -p ${target_dir};

if ${has_std}; then
    cfg_args+="--config unstable.build-std=[\"core\",\"std\"]";
fi

if ${has_linker_script}; then
    source_linker_script=${mcu_dir}/linker-script.lld;
    target_linker_script=${target_path_prefix}.lld;
    cp ${source_linker_script} ${target_linker_script};
    linker_script_args="
        --config env.SMEG_SOURCE_LINKER_SCRIPT=\"${source_linker_script}\"
        --config env.SMEG_TARGET_LINKER_SCRIPT=\"${target_linker_script}\"";
else
    linker_script_args="";
fi

if ${link_with_libc}; then
    linker_script_args+="--config env.SMEG_LINK_WITH_LIBC=\"true\"";
fi

if ${has_target_json}; then
    target_json=${target_path_prefix}.json;
    cp ${mcu_dir}/rust-target.json ${target_json};
    cp ${mcu_dir}/openocd.cfg ${target_dir}/;
    target_args="--target ${target_json}";
else
    target_args="--target-dir ${target_triplet_dir}";
fi

RUSTUP_TOOLCHAIN=nightly;
cargo +${RUSTUP_TOOLCHAIN} build -v \
    --profile ${build_profile} \
    ${target_args} \
    ${cfg_args} \
    --features smeg-board-${board_manufacturer}-${board_name}-${board_configuration} \
    ${linker_script_args} || exit 3;

if ${has_flash_image}; then
    flash_sections="-j .text -j .text.* -j .data -j .data.*";
    image_prefix="${target_dir}/smeg-os";
    echo;

    echo "Generating ${image_prefix}.hex from ${image_prefix}.elf...";
    rust-objcopy ${flash_sections} -O ihex ${image_prefix}.elf ${image_prefix}.hex || exit 4;

    echo "Generating ${image_prefix}.bin from ${image_prefix}.elf...";
    rust-objcopy ${flash_sections} -O binary ${image_prefix}.elf ${image_prefix}.bin || exit 5;
fi;
