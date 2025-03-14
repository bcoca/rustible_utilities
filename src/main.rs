mod module_utils;

use module_utils::{ArgDefaults};


ModuleArgs! {struct TestModuleArgs {yolo: String}}

fn main() {

    let t = TestModuleArgs("lola");
    println!("{:?}", t);
}
