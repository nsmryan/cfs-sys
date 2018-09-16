extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
        let bindings =
            bindgen::Builder::default()
                             // CFE Mission Includes
                             .header("cFE/cfe/fsw/mission_inc/cfe_mission_cfg.h")

                             // CFE Platform Includes
                             .header("cFE/cfe/fsw/platform_inc/cpu1/cfe_msgids.h")
                             .header("cFE/cfe/fsw/platform_inc/cpu1/cfe_platform_cfg.h")

                             // CFE Includes
                             .header("cFE/cfe/fsw/cfe-core/src/inc/ccsds.h")
                             .header("cFE/cfe/fsw/cfe-core/src/inc/cfe_error.h")
                             .header("cFE/cfe/fsw/cfe-core/src/inc/cfe_es_events.h")
                             .header("cFE/cfe/fsw/cfe-core/src/inc/cfe_es.h")
                             .header("cFE/cfe/fsw/cfe-core/src/inc/cfe_es_msg.h")
                             .header("cFE/cfe/fsw/cfe-core/src/inc/cfe_es_perfids.h")
                             .header("cFE/cfe/fsw/cfe-core/src/inc/cfe_evs_events.h")
                             .header("cFE/cfe/fsw/cfe-core/src/inc/cfe_evs.h")
                             .header("cFE/cfe/fsw/cfe-core/src/inc/cfe_evs_msg.h")
                             .header("cFE/cfe/fsw/cfe-core/src/inc/cfe_fs.h")
                             .header("cFE/cfe/fsw/cfe-core/src/inc/cfe.h")
                             .header("cFE/cfe/fsw/cfe-core/src/inc/cfe_sb_events.h")
                             .header("cFE/cfe/fsw/cfe-core/src/inc/cfe_sb.h")
                             .header("cFE/cfe/fsw/cfe-core/src/inc/cfe_sb_msg.h")
                             .header("cFE/cfe/fsw/cfe-core/src/inc/cfe_tbl_events.h")
                             .header("cFE/cfe/fsw/cfe-core/src/inc/cfe_tbl_filedef.h")
                             .header("cFE/cfe/fsw/cfe-core/src/inc/cfe_tbl.h")
                             .header("cFE/cfe/fsw/cfe-core/src/inc/cfe_tbl_msg.h")
                             .header("cFE/cfe/fsw/cfe-core/src/inc/cfe_time_events.h")
                             .header("cFE/cfe/fsw/cfe-core/src/inc/cfe_time.h")
                             .header("cFE/cfe/fsw/cfe-core/src/inc/cfe_time_msg.h")
                             .header("cFE/cfe/fsw/cfe-core/src/inc/cfe_version.h")
                             .header("cFE/cfe/fsw/cfe-core/src/inc/network_includes.h")

                             // PSP Includes
                             .header("cFE/psp/fsw/inc/cfe_psp_configdata.h")
                             .header("cFE/psp/fsw/inc/cfe_psp.h")
                             .header("cFE/psp/fsw/pc-linux/inc/cfe_psp_config.h")
                             .header("cFE/psp/fsw/pc-linux/inc/psp_version.h")

                             // OSAL Includes
                             .header("cFE/osal/src/os/inc/common_types.h")
                             .header("cFE/osal/src/os/inc/osapi.h")
                             .header("cFE/osal/src/os/inc/osapi-os-core.h")
                             .header("cFE/osal/src/os/inc/osapi-os-filesys.h")
                             .header("cFE/osal/src/os/inc/osapi-os-loader.h")
                             .header("cFE/osal/src/os/inc/osapi-os-net.h")
                             .header("cFE/osal/src/os/inc/osapi-os-timer.h")
                             .header("cFE/osal/src/os/inc/osapi-version.h")

                             // Clang Arguments

                             // Include Path CFE
                             .clang_arg("-IcFE/cfe/fsw/cfe-core/src/inc")
                             .clang_arg("-IcFE/cfe/fsw/mission_inc")
                             .clang_arg("-IcFE/cfe/fsw/platform_inc/cpu1")

                             // Include Path PSP
                             .clang_arg("-IcFE/psp/fsw/inc")
                             .clang_arg("-IcFE/psp/fsw/pc-linux/inc")

                             // Include Path OSAL
                             .clang_arg("-IcFE/osal/src/os/inc")
                             .clang_arg("-IcFE/osal/src/bsp/pc-linux/config")

                             // Define operating system for network_includes.h in CFE
                             .clang_arg("-D_LINUX_OS_")

                             .generate()
                             .expect("Unable to generate bindings!");

        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

        bindings.write_to_file(out_path.join("bindings.rs"))
                .expect("Couldn't write bindings");
}
