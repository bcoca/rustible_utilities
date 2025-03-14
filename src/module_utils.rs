// extern crate phf;
extern crate serde;

// use std::collections::HashSet;
// use std::env;
// use std::fs::File;
// use std::o::prelude::*;
// use std::process;
// use serde::{Serialize, Deserialize};
use std::path::{Path, PathBuf};

const DATE_FORMAT_STR: &'static str = "%Y-%m-%d  %H:%M:%S";

pub struct ArgDefaults {}
impl ArgDefaults {
    pub fn r#false() -> bool {false}
    pub fn sha1() -> String {"sha1".to_string()}
    // pub fn d_selinux_fs() -> Vec<str> {return vec!["fuse", "nfs", "vboxsf", "ramfs", "9p", "vfat"];}
    pub fn shell() -> String {"/bin/sh".to_string()}
    pub fn syslog_facility() -> String {"INFO".to_string()}
    pub fn r#true() -> bool {true}
    pub fn v() -> u32 {0}
    pub fn version() -> String {"0.0".to_string()}
}

#[derive(serde::Serialize)]
pub struct Deprecation {
    why: String,
    alternatives: String,
    version: Option<String>,
    date: Option<String>,
    collection_name: Option<String>,
}

#[macro_export]
macro_rules! ModuleArgs {
    ($(#[$struct_meta:meta])*
    $pub:vis struct $name:ident { $($fpub:vis $field:ident : $type:ty), *}
//    (#[derive($($derive:meta),*)] $pub:vis struct $name:ident { $($fpub:vis $field:ident : $type:ty,)* }
    ) => {
//        #[derive($($derive),*)]
		#[derive(serde::Deserialize, Default)]
		#[allow(dead_code)]
        $pub struct $name {

			// ansible common
			#[serde(alias = "_ansible_check_mode", default = "ArgDefaults::false")]
			check_mode: bool,
			#[serde(alias = "_ansible_debug", default = "ArgDefaults::false")]
			debug: bool,
			#[serde(alias = "_ansible_diff", default = "ArgDefaults::false")]
			diff: bool,
			#[serde(alias = "_ansible_keep_remote_files", default = "ArgDefaults::false")]
			keep_remote_files: bool,
			#[serde(alias = "_ansible_ignore_unknown_opts", default = "ArgDefaults::false")]
			ignore_unknown_opts: bool, // normally use #[serde(deny_unknown_fields)] but we want this at runtime?
			#[serde(alias = "_ansible_module_name")]
			module_name: String,
			#[serde(alias = "_ansible_no_log", default = "ArgDefaults::false")]
			no_log: bool,
			#[serde(alias = "_ansible_remote_tmp")]
			remote_tmp: Option<String>,
			#[serde(alias = "_ansible_target_log_info")]
			target_log_info: Option<String>,
			//    #[serde(alias = "_ansible_selinux_special_fs", default = "d_selinux_fs")]
			//    selinux_special_fs: Vec<str>,
			#[serde(alias = "_ansible_shell_executable", default = "d_shell")]
			shell_executable: String,
			#[serde(alias = "_ansible_socket_path")]
			socket: Option<String>,
			#[serde(alias = "_ansible_syslog_facility", default = "d_syslog_facility")]
			syslog_facility: String,
			#[serde(alias = "_ansible_tmpdir")]
			tmpdir: Option<String>,
			#[serde(alias = "_ansible_verbosity", default = "d_v")]
			verbosity: u32,
			#[serde(alias = "_ansible_version", default = "d_version")]
			version: String,

			// module specific
            $($fpub $field : $type,)*
		}

		impl $name {

			$pub fn new($($field:$type,)*) -> Self {
                Self {
                    $($field,)*
                    ..Self::default()
                }
			}

			fn from_argsfile(path: &Path) -> $type {
			    match std::fs::read_to_string(path) {
			        Ok(file_contents) => {
			                let args: $type = match serde_json::from_str(&file_contents) {
			                    Ok(data) => { data },
			                    Err(e) => {panic!("Unable to parse the provided arguments file ({:?}) as JSON: {:?}", path, e)},
			                };
			                return args;
			        },
			        Err(e) => {
			            // TODO: fail_json/raise error?
			            panic!("Unable to read the provided arguments file({:?}): {:?} !", path, e);
			        },
			    };
			}
		}
	}
}

#[macro_export]
macro_rules! ModuleResult {
    (#[derive($($derive:meta),*)] $pub:vis struct $name:ident { $($fpub:vis $field:ident : $type:ty,)* }) => {
        #[derive(serde::Serialize, Default)]
        #[derive($($derive),*)]
        $pub struct $name {
            // required for all Results
			msg: Option<String>,
			changed: bool,
			failed: bool,
            // Internal, set from ModuleArgs
            #[serde(skip_serializing)]
            debug: bool,
            #[serde(skip_serializing)]
            module_name: String,

            // convey non fatal errors
            warnings: HashSet<String>,
            deprecations: HashSet<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
			traceback: Option<string>, // TODO; set on fail_json?

            $($fpub $field : $type,)*
        }
        impl $name {
            $pub fn new(msg: Option<string>, changed: bool, failed: bool, debug: bool, $($field:$type,)*) -> Self{
                Self{
                    msg,
					changed,
					failed,
                    debug,
                    $($field,)*
                    ..Self::default()
                }
			}

			// TODO:: add log
			fn deprecate(&mut self, deprecation: module_utils::Deprecation) {
			    if self.debug {
			        eprintln!("[DEPRECATED] {}", warning);
			    }
			    self.deprecations.insert(deprecation);
            }

			fn warn(&mut self, warning: String) {
			    if self.debug {
			        eprintln!("[WARNING] {}", warning);
			    }
			    self.warnings.insert(warning);
			}

			fn exit_json(&mut self, msg: Option<String>) {
			    self.return_result(msg);
			    process::exit(exitcode::OK);
			}

			fn fail_json(&mut self, msg: String) {
			    // TODO: populate traceback?
			    if self.debug {
			        eprintln!("{:?}", msg);
			    }
			    if self.failed.not() {
			        self.failed = true;
			    }
			    self.return_result(Some(msg));
			    process::exit(1);
			}

			fn return_result(&mut self, msg: Option<String>) {
			    self.msg = msg;
			    if self.debug {
			        println!("{}", serde_json::to_string_pretty(&self).unwrap());
			    } else {
			        println!("{}", serde_json::to_string(&self).unwrap());
			    }
			}

            fn debug(&self, msg: String) {
                if self.debug {
                    let module_name = "placeholder"; // TODO: pull from ModuleArgs
                    eprintln!(" [DEBUG] {} (pid:{:?}) [{}]: {:?}", module_name, process::id(), Local::now().format(DATE_FORMAT_STR).to_string(), msg);
                }
            }

		}
	}
}

fn main () {
    println!("Hello world!");
}
