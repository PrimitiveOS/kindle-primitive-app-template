use std::{fs, path::{Path, PathBuf}};

use hello_core::Hello;
use hello_render::View;
use primitive_app::{frame, font_paths, write_png, App, Change, HEIGHT, WIDTH};
use primitive_ui::{Canvas, Input};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = PathBuf::from(std::env::args_os().nth(1).unwrap_or_else(|| "build/previews".into()));
    fs::create_dir_all(&output)?;
    let fonts = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../doannt-kindle-launcher/assets/fonts");
    let mut canvas = Canvas::new(WIDTH, HEIGHT, &font_paths(&fonts))?;
    let mut app = View { hello: Hello::default(), data: None };
    write_png(&output.join("hello.png"), WIDTH, HEIGHT, &frame(&app, &mut canvas, false))?;
    assert_eq!(app.input(Input::Tap(500.0, 840.0)), Change::Partial);
    assert_eq!(app.hello.count(), 1);
    write_png(&output.join("hello-dark.png"), WIDTH, HEIGHT, &frame(&app, &mut canvas, true))?;
    Ok(())
}
