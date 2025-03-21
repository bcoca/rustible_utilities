extern crate serde;
extern crate serde_yaml;

use serde::Deserialize;
// use serde_yaml;
use std::any::Any;
// use std::env;
// use std::fs:
// use std::path::Path;

#[derive(serde::Deserialize, Default)]
struct Deprecated {
    why: str,
    version: str,
    alternatives: str,
}

macro_rules! AnsibleCommon {
    ($(#[$struct_meta:meta])*
    $pub:vis struct $name:ident { $($fpub:vis $field:ident : $type:ty), *}
    ) => {
		#[derive(serde::Deserialize, Default)]
        #[serde(deny_unknown_fields)]
		#[allow(dead_code)]
        $pub struct $name {
            name: str,
            deprecated: Deprecated,
            version_added: str,
            $($fpub $field : $type,)*
        }
    }
}
impl AnsibleCommon {
    fn new() -> Self {
        Self {
            deprecated: None
        }
    }
}

AnsibleCommon!(pub struct EnvVars{});
AnsibleCommon!(pub struct AnsibleVar{});
AnsibleCommon!(pub struct AnsibleCli{});

#[derive(serde::Deserialize, Default)]
struct IniEntry {
    key: str,
    section: str,
    deprecated: Deprecated,
    version_added: VersionAdded
}

#[serde(rename_all = "lowercase")]
#[derive(Default, Debug, Clone)]
pub enum ConfigTypes {
    Bool,
    Dict,
    Float,
    List,
    Int,
    r#None,
    Path,
    Pathspec,
    Pathlist,
    Raw,
    #[default]
    Str,
    Tmp,
}

impl ConfigTypes {
    pub const Boolean: ConfigTypes = ConfigTypes::Bool;
    pub const Dictionary: ConfigTypes = ConfigTypes::Dict;
    pub const Integer: ConfigTypes = ConfigTypes::Int;
    pub const r#String: ConfigTypes = ConfigTypes::Str;
    pub const Temppath: ConfigTypes = ConfigTypes::Tmp;
    pub const Tmppath: ConfigTypes = ConfigTypes::Tmp;
}

AnsibleCommon!(struct AnsibleSetting {
        description: str,
        r#type: ConfigTypes,
        default: Any, // default to None
        value: Any,
        env: Vec<EnvVars>,
        ini: Vec<IniEntry>,
        vars: Vec<AnsibleVar>,
    }
);

impl AnsibleSetting {
    fn new() -> AnsibleSetting {
        AnsibleSetting {
            r#type: "str",
            description: "UNDOCUMENTED",
            default: None,
            value: None,
            env: None,
            ini: None,
            vars: None,
        }
    }
}


// cfg = env!("ANSIBLE_CONFIG")
