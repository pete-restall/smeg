#!/bin/bash
set -e;
THISDIR="$(dirname "$(readlink -f "$0")")";
pushd ${THISDIR};
echo "Working directory: $(pwd)";

arg_board="${1}";
arg_build_action="${2}";
arg_build_profile="${3}";

build_action=${arg_build_action:-build} #"llvm-cov --workspace --codecov --output-path lcov.info"; # TODO: we want a command-line argument for this at some point

build_profile=${arg_build_profile:-dev}
if [ "x${build_profile}" == "xdev" ]; then
    build_profile_target=debug;
else
    build_profile_target=release;
fi;

board_manufacturer="$(echo ${arg_board} | cut -d- -f 1)";
board_name="$(echo ${arg_board} | cut -d- -f 2)";
board_configuration="$(echo ${arg_board} | cut -d- -f 3)";
board_configuration=${board_configuration:-default}
board_triplet="${board_manufacturer}-${board_name}-${board_configuration}";

has_flash_image=true;
has_std=false;
has_target_json=true;
link_with_libc=false;

if [ $# -lt 1 ] || [ "x${board_manufacturer}" == "x" ] || [ "x${board_name}" == "x" ]; then
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
    ( host-rust_std-* )
        mcu_manufacturer=host;
        mcu_name=rust_std;
        has_flash_image=false;
        has_std=true;
        has_target_json=false;
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

cfg_args="--config env.SMEG_OUT_DIR=\"${THISDIR}/${target_dir}\"";
linker_args='';
target_args='';
test_args='';

if ${has_std}; then
    cfg_args+=' --config unstable.build-std=["core","std"]';
fi

if ${link_with_libc}; then
    linker_args+=' --config env.SMEG_LINK_WITH_LIBC="true"';
fi

if ${has_target_json}; then
    target_json=${target_path_prefix}.json;
    cp ${mcu_dir}/rust-target.json ${target_json};
    cp ${mcu_dir}/openocd.cfg ${target_dir}/;
    target_args+=" --target ${target_json}";
else
    target_args+=" --target-dir ${target_triplet_dir}";
fi

if [ "x${build_action}" == "xtest" ]; then
    # TODO: 'llvm-cov' would be good, but it doesn't support '--config' flags !  There's a TODO in its source, so maybe at some point...?
    # build_action="llvm-cov";
    # test_args="--workspace --codecov --output-path lcov.info";
    # target_args="";
    test_args+=" --workspace";
fi

RUSTUP_TOOLCHAIN=nightly;
cargo +${RUSTUP_TOOLCHAIN} ${build_action} -v \
    ${test_args} \
    --profile ${build_profile} \
    ${target_args} \
    ${cfg_args} \
    --features smeg-board-${board_manufacturer}-${board_name}-${board_configuration} \
    ${linker_args} || exit 3;

if ${has_flash_image}; then
    flash_sections="-j .text -j .text.* -j .data -j .data.*";
    image_prefix="${target_dir}/smeg-os";
    echo;

    echo "Generating ${image_prefix}.hex from ${image_prefix}.elf...";
    rust-objcopy ${flash_sections} -O ihex ${image_prefix}.elf ${image_prefix}.hex || exit 4;

    echo "Generating ${image_prefix}.bin from ${image_prefix}.elf...";
    rust-objcopy ${flash_sections} -O binary ${image_prefix}.elf ${image_prefix}.bin || exit 5;
fi;
