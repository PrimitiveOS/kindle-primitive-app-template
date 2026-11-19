mod gallery;

use std::{
    fs,
    path::{Path, PathBuf},
};

use hello_core::Hello;
use hello_render::View;
use primitive_app::{font_paths, frame, write_png, Change, Lifecycle, HEIGHT, WIDTH};
use primitive_ui::{changed_rect, for_panel, Canvas, Input, Rotation};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = PathBuf::from(
        std::env::args_os()
            .nth(1)
            .unwrap_or_else(|| "build/previews".into()),
    );
    fs::create_dir_all(&output)?;
    let fonts =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../doannt-kindle-launcher/assets/fonts");
    let mut canvas = Canvas::new(WIDTH, HEIGHT, &font_paths(&fonts))?;
    let scratch = std::env::temp_dir().join(format!("primitive-hello-{}", std::process::id()));
    fs::create_dir(&scratch)?;
    let data = scratch.join("greetings.txt");
    let mut app = View {
        hello: Hello::default(),
        data: Some(data.clone()),
    };
    let mut lifecycle = Lifecycle::default();
    let on_error = |error| panic!("Hello could not save its simulated state: {error}");
    write_png(
        &output.join("hello.png"),
        WIDTH,
        HEIGHT,
        &frame(&app, &mut canvas, false),
    )?;
    let greet = Input::Tap(500.0, 840.0);
    assert_eq!(lifecycle.input(&mut app, greet, on_error), Change::Partial);
    assert_eq!(
        lifecycle.input(&mut app, Input::Sleep, on_error),
        Change::None
    );
    assert_eq!(Hello::restore(&fs::read(&data)?).count(), 1);
    assert_eq!(lifecycle.input(&mut app, greet, on_error), Change::None);
    assert_eq!(
        lifecycle.input(&mut app, Input::Wake, on_error),
        Change::Full
    );
    write_png(
        &output.join("hello-dark.png"),
        WIDTH,
        HEIGHT,
        &frame(&app, &mut canvas, true),
    )?;
    assert_eq!(lifecycle.input(&mut app, greet, on_error), Change::Partial);
    assert_eq!(
        lifecycle.input(&mut app, Input::Tap(1124.0, 90.0), on_error),
        Change::Exit
    );
    assert_eq!(Hello::restore(&fs::read(&data)?).count(), 2);
    fs::remove_file(&data)?;
    fs::remove_dir(&scratch)?;
    let controls = gallery::controls(&mut canvas, false, None);
    write_png(&output.join("components.png"), WIDTH, HEIGHT, &controls)?;
    let pressed = gallery::controls(&mut canvas, false, Some(1));
    let (x, y, width, height) = changed_rect(&controls, &pressed, WIDTH as usize)
        .expect("pressing a component changes its pixels");
    let row = gallery::PRESSED_ROW;
    assert!(x >= row.x as usize && x + width <= row.right() as usize);
    assert!(y >= row.y as usize && y + height <= row.bottom() as usize);
    write_png(
        &output.join("components-pressed.png"),
        WIDTH,
        HEIGHT,
        &pressed,
    )?;
    write_png(
        &output.join("components-upside-down.png"),
        WIDTH,
        HEIGHT,
        &for_panel(controls, Rotation::UpsideDown, 0),
    )?;
    for (name, image) in [
        (
            "components-dark.png",
            gallery::controls(&mut canvas, true, None),
        ),
        ("components-sheet.png", gallery::sheet(&mut canvas, false)),
        (
            "components-sheet-dark.png",
            gallery::sheet(&mut canvas, true),
        ),
        (
            "components-progress.png",
            gallery::install_progress(&mut canvas, false),
        ),
        (
            "components-keyboard.png",
            gallery::keyboard(&mut canvas, false),
        ),
        (
            "components-message.png",
            gallery::message(&mut canvas, false),
        ),
    ] {
        write_png(&output.join(name), WIDTH, HEIGHT, &image)?;
    }
    Ok(())
}
