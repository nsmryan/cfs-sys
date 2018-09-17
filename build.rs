extern crate bindgen;

use std::env;
use std::path::PathBuf;
<<<<<<< HEAD
use std::ffi::OsStr;

// look up an environment variable and prefix its value to a given path
pub fn with_dir(dir : &str, path : &str) -> String {
    let dir_os_string  = OsStr::new(dir);
    let path_os_string = OsStr::new(path);
    let full_path = PathBuf::from(env::var(dir_os_string).unwrap());
    String::from(full_path.join(path_os_string).to_str().unwrap())
}

// look up an environment variable and prefix its value to a given path,
// resulting in an include "-IPATH"
pub fn with_include(dir : &str, path : &str) -> String {
    let dir_os_string  = OsStr::new(dir);
    let path_os_string = OsStr::new(path);
    let full_path = PathBuf::from(env::var(dir_os_string).unwrap());
    let joined_path = String::from(full_path.join(path_os_string).to_str().unwrap());
    let mut include = "-I".to_string();
    include.push_str(&joined_path);

    include
}

// look up an environment variable and turn its value into an include "-I"
pub fn as_include(dir : &str) -> String {
    let dir_os_string  = OsStr::new(dir);
    let full_path = PathBuf::from(env::var(dir_os_string).unwrap());
    let mut include = "-I".to_string();
    include.push_str(&full_path.to_str().unwrap());
    println!("{}", include.as_str());

    include
}

fn main() {
    let bindings = bindgen::Builder::default()
          // CFE Mission Includes
          .header(with_dir("CFS_MISSION", "build/mission_inc/cfe_mission_cfg.h"))

          // CFE Platform Includes
          .header(with_dir("CFE_FSW", "platform_inc/cpu1/cfe_msgids.h"))
          .header(with_dir("CFE_FSW", "platform_inc/cpu1/cfe_platform_cfg.h"))

          // CFE Includes
          .header(with_dir("CFE_CORE_SRC", "inc/ccsds.h"))
          .header(with_dir("CFE_CORE_SRC", "inc/cfe_error.h"))
          .header(with_dir("CFE_CORE_SRC", "inc/cfe_es_events.h"))
          .header(with_dir("CFE_CORE_SRC", "inc/cfe_es.h"))
          .header(with_dir("CFE_CORE_SRC", "inc/cfe_es_msg.h"))
          .header(with_dir("CFE_CORE_SRC", "inc/cfe_es_perfids.h"))
          .header(with_dir("CFE_CORE_SRC", "inc/cfe_evs_events.h"))
          .header(with_dir("CFE_CORE_SRC", "inc/cfe_evs.h"))
          .header(with_dir("CFE_CORE_SRC", "inc/cfe_evs_msg.h"))
          .header(with_dir("CFE_CORE_SRC", "inc/cfe_fs.h"))
          .header(with_dir("CFE_CORE_SRC", "inc/cfe.h"))
          .header(with_dir("CFE_CORE_SRC", "inc/cfe_sb_events.h"))
          .header(with_dir("CFE_CORE_SRC", "inc/cfe_sb.h"))
          .header(with_dir("CFE_CORE_SRC", "inc/cfe_sb_msg.h"))
          .header(with_dir("CFE_CORE_SRC", "inc/cfe_tbl_events.h"))
          .header(with_dir("CFE_CORE_SRC", "inc/cfe_tbl_filedef.h"))
          .header(with_dir("CFE_CORE_SRC", "inc/cfe_tbl.h"))
          .header(with_dir("CFE_CORE_SRC", "inc/cfe_tbl_msg.h"))
          .header(with_dir("CFE_CORE_SRC", "inc/cfe_time_events.h"))
          .header(with_dir("CFE_CORE_SRC", "inc/cfe_time.h"))
          .header(with_dir("CFE_CORE_SRC", "inc/cfe_time_msg.h"))
          .header(with_dir("CFE_CORE_SRC", "inc/cfe_version.h"))
          .header(with_dir("CFE_CORE_SRC", "inc/network_includes.h"))

          // PSP Includes
          .header(with_dir("PSP_DIR", "fsw/inc/cfe_psp_configdata.h"))
          .header(with_dir("PSP_DIR", "fsw/inc/cfe_psp.h"))
          .header(with_dir("PSP_DIR", "fsw/pc-linux/inc/cfe_psp_config.h"))
          .header(with_dir("PSP_DIR", "fsw/pc-linux/inc/psp_version.h"))

          // OSAL Includes
          .header(with_dir("OSAL_DIR", "src/os/inc/common_types.h"))
          .header(with_dir("OSAL_DIR", "src/os/inc/osapi.h"))
          .header(with_dir("OSAL_DIR", "src/os/inc/osapi-os-core.h"))
          .header(with_dir("OSAL_DIR", "src/os/inc/osapi-os-filesys.h"))
          .header(with_dir("OSAL_DIR", "src/os/inc/osapi-os-loader.h"))
          .header(with_dir("OSAL_DIR", "src/os/inc/osapi-os-net.h"))
          .header(with_dir("OSAL_DIR", "src/os/inc/osapi-os-timer.h"))
          .header(with_dir("OSAL_DIR", "src/os/inc/osapi-version.h"))

          // Clang Arguments

          // Include Path CFE
          .clang_arg(with_include("CFE_CORE_SRC", "fsw/cfe-core/src/inc"))
          .clang_arg(as_include("CFS_MISSION_INC"))
          .clang_arg(with_include("CFS_MISSION", "build/mission_inc"))
          .clang_arg(with_include("CFE_CORE_SRC", "fsw/platform_inc/cpu1"))
          .clang_arg(with_include("CFS_MISSION", "build/cpu1/inc"))

          // Include Path PSP
          .clang_arg(with_include("PSP_DIR", "fsw/inc"))
          .clang_arg(with_include("PSP_DIR", "fsw/pc-linux/inc"))

          // Include Path OSAL
          .clang_arg(with_include("OSAL_DIR", "src/os/inc"))
          .clang_arg(with_include("OSAL_DIR", "src/bsp/pc-linux/config"))

          // Define operating system for network_includes.h in CFE
          .clang_arg("-D_LINUX_OS_")

          .generate()
          .expect("Unable to generate bindings!");

    // write out bindings to the source directory, where they are re-exported.
    bindings.write_to_file("src/bindings.rs").expect("Couldn't write bindings");

=======

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
>>>>>>> be19f9f8f42ef8ec63ab56f0f1fc2f5676aa54e7
}
