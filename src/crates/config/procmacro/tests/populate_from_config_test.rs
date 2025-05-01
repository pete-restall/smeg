#![allow(non_snake_case)]

use fluent_test::prelude::*;

use smeg_config_procmacro::populate_from_config;

#[test]
fn populate_from_config__called_when_only_override_config_present__expect_same_config_from_that_single_file() {
    #[populate_from_config(workspace_dir = "${workspace_dir}/crates/config/procmacro/tests/fixtures/single_config")]
    struct Config;

    let config = Config::default();
    expect!(&config.VALUES).to_equal(&Config_generated::Config {
        AN_ARRAY_OF_ARRAYS: [[1, 2, 3], [4, 5, 6]],
        AN_ARRAY_OF_INTEGERS: [1, 2, 3],
        AN_ARRAY_OF_FLOATS: [1.23, 4.56, 7.8e9],
        AN_ARRAY_OF_STRINGS: ["hello", "world"],
        AN_ARRAY_OF_STRUCTS: [
            Config_generated::Config_AnArrayOfStructs_0 { A: "a", B: 2, C: 3.0 },
            Config_generated::Config_AnArrayOfStructs_0 { A: "A", B: -2, C: -3.0 }
        ],
        AN_INTEGER: 123,
        A_FLOAT: -1.23,
        A_STRING: "foo",
        A_NESTED_STRUCT: Config_generated::Config_ANestedStruct {
            FOO: Config_generated::Config_ANestedStruct_Foo { BAR: "baz" }
        },
        A_STRUCT: Config_generated::Config_AStruct { FOO: "bar", BAZ: 42 }
    });
}

#[test]
fn populate_from_config__called_when_non_smeg_config_tomls_exist__expect_only_smeg_config_files_are_included() {
    #[populate_from_config(workspace_dir = "${workspace_dir}/crates/config/procmacro/tests/fixtures/superfluous_config")]
    struct Config;

    let config = Config::default();
    expect!(&config.VALUES).to_equal(&Config_generated::Config {
        I_HOLD: "smeg config",
        THIS_FILE_IS_PROPER_CONFIG: true
    });
}

#[test]
fn populate_from_config__called_when_hierarchical_smeg_config_tomls_exist__expect_more_specific_config_overrides_more_general_config() {
    #[populate_from_config(workspace_dir = "${workspace_dir}/crates/config/procmacro/tests/fixtures/simple_hierarchical_config")]
    struct Config;

    let config = Config::default();
    expect!(&config.VALUES).to_equal(&Config_generated::Config {

        KERNEL_NOT_OVERRIDDEN: "some config for the kernel",
        KERNEL_OVERRIDDEN_IN_MCU_FAMILY: "set in the MCU family config",
        KERNEL_OVERRIDDEN_IN_MCU: "MCU laying down configs",
        KERNEL_OVERRIDDEN_IN_DRIVER_FAMILY: "was kernel config, now driver family config",
        KERNEL_OVERRIDDEN_IN_DRIVER: "driver pwns kernel config",
        KERNEL_OVERRIDDEN_IN_BOARD_FAMILY: "all yours kernel configs are belong to us",
        KERNEL_OVERRIDDEN_IN_BOARD: "yeah, overridden by the board config",
        KERNEL_OVERRIDDEN_IN_ROOT: "root !",

        MCU_FAMILY_NOT_OVERRIDDEN: "some config for an MCU family",
        MCU_FAMILY_NOT_OVERRIDDEN_ABOVE: "MCU family",
        MCU_FAMILY_OVERRIDDEN_IN_MCU: "something something something MCU-side",
        MCU_FAMILY_OVERRIDDEN_IN_DRIVER_FAMILY: "was MCU family, now driver family config",
        MCU_FAMILY_OVERRIDDEN_IN_DRIVER: "driver config conquers MCU family",
        MCU_FAMILY_OVERRIDDEN_IN_BOARD_FAMILY: "the board family is more specific than the MCU family",
        MCU_FAMILY_OVERRIDDEN_IN_BOARD: "originally from MCU family config, now part of the board config",
        MCU_FAMILY_OVERRIDDEN_IN_ROOT: "another win for root config",

        MCU_NOT_OVERRIDDEN: "some config for an MCU",
        MCU_NOT_OVERRIDDEN_ABOVE: "MCU",
        MCU_OVERRIDDEN_IN_DRIVER_FAMILY: "was MCU, now driver family config",
        MCU_OVERRIDDEN_IN_DRIVER: "driver overrides MCU config",
        MCU_OVERRIDDEN_IN_BOARD_FAMILY: "the board family is more specific than the MCU",
        MCU_OVERRIDDEN_IN_BOARD: "originally from MCU config and then overridden by the board config",
        MCU_OVERRIDDEN_IN_ROOT: "yup, it's still root",

        DRIVER_FAMILY_NOT_OVERRIDDEN: "some config for a driver family",
        DRIVER_FAMILY_NOT_OVERRIDDEN_ABOVE: "driver family",
        DRIVER_FAMILY_OVERRIDDEN_IN_DRIVER: "driver trumps driver family",
        DRIVER_FAMILY_OVERRIDDEN_IN_BOARD_FAMILY: "the board family is more specific than the driver family",
        DRIVER_FAMILY_OVERRIDDEN_IN_BOARD: "from board config, overriding something in driver family config",
        DRIVER_FAMILY_OVERRIDDEN_IN_ROOT: "rooty mcrootface",

        DRIVER_NOT_OVERRIDDEN: "some config for a driver",
        DRIVER_NOT_OVERRIDDEN_ABOVE: "driver",
        DRIVER_OVERRIDDEN_IN_BOARD_FAMILY: "the board family is more specific than the driver",
        DRIVER_OVERRIDDEN_IN_BOARD: "from driver config and now overridden by the board config",
        DRIVER_OVERRIDDEN_IN_ROOT: "root all the way down...",

        BOARD_FAMILY_NOT_OVERRIDDEN: "some config for a board family",
        BOARD_FAMILY_NOT_OVERRIDDEN_ABOVE: "board family",
        BOARD_FAMILY_OVERRIDDEN_IN_BOARD: "this comes from the board config",
        BOARD_FAMILY_OVERRIDDEN_IN_ROOT: "root",

        BOARD_NOT_OVERRIDDEN: "some config for a board",
        BOARD_NOT_OVERRIDDEN_ABOVE: "board",
        BOARD_OVERRIDDEN_IN_ROOT: "root again",

        ROOT_NOT_OVERRIDDEN: "some config for the root",
        ROOT_NOT_OVERRIDDEN_ANYWHERE: "definitely can't be overridden..."
    });
}

// TODO: Test various name transformations; some-name -> SOME_NAME, some_name -> SOME_NAME, etc.
// TODO: Test sections; [some-section], [some section], [[some-section]], ["some section"], etc.
// TODO: Test section overrides
