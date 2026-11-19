use std::{fs, path::PathBuf};

use hello_core::Hello;
use hello_render::View;

fn main() -> Result<(), String> {
    let dir = PathBuf::from("/mnt/us/hello");
    let data = dir.join("greetings.txt");
    let app = View { hello: Hello::restore(&fs::read(&data).unwrap_or_default()), data: Some(data) };
    primitive_app::run(app, &dir)
}
