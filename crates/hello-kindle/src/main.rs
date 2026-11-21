use std::{fs, path::PathBuf};

use hello_core::Hello;
use hello_render::View;

fn main() -> Result<(), String> {
    let dir = PathBuf::from("/mnt/us/hello");
    let data = dir.join("greetings.txt");
    let app = View { hello: Hello::restore(&fs::read(&data).unwrap_or_default()), data: Some(data) };
    // Under Primitive Launcher the kit redraws on SIGUSR1 in the system theme, without a flash, and leaves the swipe
    // down from the top edge to the launcher. The app has no sleep, quick settings or light and dark code of its own.
    primitive_app::run(app, &dir)
}
