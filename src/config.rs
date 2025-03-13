use serde::Deserialize;
use serde_yaml;
use std::any::Any;
use Vec;

struct VersionAdded(&str);
struct Deprecated {
    why: &str,
    version: &str,
    alternatives: &str,
}

struct AnsibleCommon {
    name: &str,
    deprecated: Deprecated,
    version_added: VersionAdded
}

impl AnsibleCommon {
    fn new -> AnsibleCommon {
        AnsibleCommon {
            deprecated = None
        }
    }
}

struct EnvVars: AnsibleCommon;
struct AnsibleVar: AnsibleCommon;
struct AnsibleCli: AnsibleCommon;
struct IniEntry {
    key: &str,
    section: &str,
    deprecated: Deprecated,
    version_added: VersionAdded
}

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

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnsibleSetting: AnsibleCommon {
    description: &str,
    r#type: ConfigTypes,
    default: Any, // default to None
    value: Any,
    env: Vec<EnvVars>,
    ini: Vec<IniEntry>,
    vars: Vec<AnsibleVar>,
}

impl AnsibleSetting {
    fn new -> AnsibleSetting {
        AnsibleSetting {
            r#type = "str",
            description = "UNDOCUMENTED",
            default = None,
            value = None,
            env = None,
            ini = None,
            vars = None,
        }
    }
}


/// cfg = env!("ANSIBLE_CONFIG")
