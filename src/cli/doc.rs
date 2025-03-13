use clap::{Parser, ValueEnum};
use serde_json::json;
use std::io::IsTerminal;

#[derive(ValueEnum, Clone, Default, Debug,)]
// #[serde(rename_all = "lowercase")]
enum PluginTypes {
	Become,
	Cache,
	Callback,
	Cliconf,
	Connection,
	Httpapi,
	Inventory,
	Lookup,
	Netconf,
	Shell,
	Vars,
	#[default]
	Module,
	Strategy,
	Role,
	Keyword
}

#[derive(Parser, Debug)]
struct CliArgs {

//  --metadata-dump       **For internal testing only** Dump json metadata for all plugins.
//  --playbook-dir BASEDIR
//                        Since this tool does not use playbooks, use this as a substitute playbook directory.This sets the relative path for many features including
//                        roles/ group_vars/ etc.
//  --version             show program's version number, config file location, configured module search path, module location, executable location and exit
//  -M MODULE_PATH, --module-path MODULE_PATH
//                        prepend colon-separated path(s) to module library (default=~/.ansible/plugins/modules:/usr/share/ansible/plugins/modules)
//  -e ENTRY_POINT, --entry-point ENTRY_POINT
//                        Select the entry point for role(s).
//  -h, --help            show this help message and exit
//  -r ROLES_PATH, --roles-path ROLES_PATH
//                        The path to the directory containing your roles.
//  -s, --snippet         Show playbook snippet for these plugin types: inventory, lookup, module
//  -v, --verbose         verbose mode (-vvv for more, -vvvv to enable connection debugging)

    // plugin name (or collection when listing)
    plugin: String,

    // Choose which plugin type (defaults to "module"). Available plugin types are : ('become', 'cache', 'callback', 'cliconf', 'connection', 'httpapi', 'inventory', 'lookup', 'netconf', 'shell', 'vars', 'module', 'strategy', 'role', 'keyword')
    #[arg(long = "type", short, default_value_t, value_enum)]
    plugin_type: PluginTypes,

    // List available plugins. A supplied argument will be used for filtering, can be a namespace or full collection name.
    #[arg(short, long)]
    list: bool,

    // Show plugin names and their source files without summaries (implies --list). A supplied argument will be used for filtering, can be a namespace or full collection name.
    // #[arg(short = "F", long)]
    // list_files: bool,

    // Change output into JSON format.
    #[arg(short, long)]
    json: bool,
}

fn main() -> std::io::Result<(), String> {
    //let plugin = std::env::args().nth(1).expect("No argument given");
    let args = CliArgs::parse();

    let mut result: String = "".to_owned();
    if args.list {
        result.push_str("implement list");
        if args.plugin != "" {
            result.push_str(&format!("option {}", args.plugin));
        }
    }
    else {
        if args.plugin != "" {
            result.push_str(&format!("Hello, world!: {}", args.plugin));
        }
        else {
            return Err("If not listing, plugin name is required.")
        }
    }
    if args.json || !std::io::stdout().is_terminal() {
        result.push_str(&format!("{}", json!({"msg": "Hello, world!", "option": args.plugin})));
    }

    // all is well, print result and exit.
    println!("{result}");
    Ok(())
}
