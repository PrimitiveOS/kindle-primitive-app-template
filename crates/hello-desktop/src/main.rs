mod gallery;

use std::{
    fs,
    path::{Path, PathBuf},
};

use hello_core::Hello;
use hello_render::View;
use primitive_app::{font_paths, frame, write_png, Change, Lifecycle, HEIGHT, WIDTH};
use primitive_ui::{
    changed_rect, for_panel, Canvas, Input, Key, Layer, Layout, Rect, Rotation, Sheet, SheetAction,
};

fn assert_feedback_rect(before: &[u8], pressed: &[u8], target: Rect) {
    let (x, y, width, height) =
        changed_rect(before, pressed, WIDTH as usize).expect("pressed feedback changes pixels");
    assert!(x >= target.x as usize / 8 * 8);
    assert!(x + width <= (target.right() as usize).div_ceil(8) * 8);
    assert!(y >= target.y as usize && y + height <= target.bottom() as usize);
}

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
    let hello = frame(&app, &mut canvas, false);
    write_png(&output.join("hello.png"), WIDTH, HEIGHT, &hello)?;
    let greet = Input::Tap(500.0, 840.0);
    assert!(lifecycle
        .press_feedback(&app, &mut canvas, Input::Tap(600.0, 600.0), false)
        .is_none());
    assert!(lifecycle
        .press_feedback(&app, &mut canvas, Input::LongTap(500.0, 840.0), false)
        .is_none());
    let pressed = lifecycle
        .press_feedback(&app, &mut canvas, greet, false)
        .expect("greeting button reacts to its tap");
    assert_feedback_rect(&hello, &pressed, Rect::new(412, 795, 440, 100));
    assert_eq!(app.hello.count(), 0);
    assert_eq!(frame(&app, &mut canvas, false), hello);
    write_png(&output.join("hello-pressed.png"), WIDTH, HEIGHT, &pressed)?;
    assert_eq!(lifecycle.input(&mut app, greet, on_error), Change::Partial);
    assert_eq!(
        lifecycle.input(&mut app, Input::Sleep, on_error),
        Change::None
    );
    assert_eq!(Hello::restore(&fs::read(&data)?).count(), 1);
    assert!(lifecycle
        .press_feedback(&app, &mut canvas, greet, false)
        .is_none());
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

    let keyboard = gallery::keyboard(&mut canvas, false);
    let layout = Layout::new(WIDTH, HEIGHT, 4, 5);
    let (_, letter) = layout
        .keyboard_keys_with(Layer::Lower, "Done")
        .into_iter()
        .find(|(key, _)| key.key == Key::Char('a'))
        .expect("keyboard includes a");
    assert!(canvas.press_at((letter.x + 1) as f32, (letter.y + 1) as f32));
    let pressed_key = gallery::keyboard(&mut canvas, false);
    assert!(canvas.is_control_pressed(letter));
    assert_feedback_rect(&keyboard, &pressed_key, letter);
    canvas.clear_press();

    let sheet = gallery::sheet(&mut canvas, false);
    assert!(
        !canvas.press_at(1124.0, 90.0),
        "the modal blocks the underlying Done button"
    );
    let confirm = Sheet::centered(canvas.bounds(), "", "", "").button(SheetAction::Confirm);
    assert!(canvas.press_at((confirm.x + 1) as f32, (confirm.y + 1) as f32));
    let pressed_confirm = gallery::sheet(&mut canvas, false);
    assert_feedback_rect(&sheet, &pressed_confirm, confirm);
    canvas.clear_press();

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
