use primitive_ui::{
    full_screen_message, header, icons, progress, section_header, Canvas, Editor, GrayImage, Layer,
    Pager, Rect, Row, RowAccessory, Rows, Sheet, Status, Tab, TabBar, TextStyle, CAPTION, TITLE,
};

const ROWS: Rect = Rect::new(40, 530, 1184, 360);

pub fn controls(canvas: &mut Canvas, dark: bool, pressed: Option<usize>) -> Vec<u8> {
    canvas.begin();
    canvas.status_bar(
        &Status {
            clock: "9:41".into(),
            date: "Thu, Nov 19".into(),
            battery: Some(78),
            charging: true,
            wifi_on: true,
            wifi_state: "connected".into(),
            wifi_signal: Some((3, 4)),
            ssh: true,
            ..Status::default()
        },
        0,
    );
    canvas.text(Rect::new(40, 176, 900, 120), "Interface components", TITLE);
    canvas.button(Rect::new(1004, 165, 220, 100), "Done", true, false);
    canvas.button(Rect::new(40, 340, 364, 100), "Primary", true, false);
    canvas.button(Rect::new(450, 340, 364, 100), "Secondary", false, false);
    canvas.button(Rect::new(860, 340, 364, 100), "Pressed", false, true);

    section_header(canvas, Rect::new(40, 474, 1184, 56), "Display");
    Rows {
        bounds: ROWS,
        rows: &[
            // A sample switch row. On the Kindle, an app under Primitive Launcher has no dark mode switch of its own:
            // the theme is the system's.
            Row {
                title: "Dark mode",
                detail: Some("White text on a dark background"),
                accessory: RowAccessory::Toggle(dark),
            },
            Row {
                title: "Text size",
                detail: Some("Standard"),
                accessory: RowAccessory::Disclosure,
            },
            Row {
                title: "Orientation",
                detail: Some("Auto-rotate"),
                accessory: RowAccessory::None,
            },
        ],
    }
    .draw(canvas, pressed);

    Pager {
        bounds: Rect::new(40, 1290, 1184, 100),
        page: 1,
        pages: 3,
    }
    .draw(canvas, None);
    TabBar {
        bounds: Rect::new(40, 1490, 1184, 130),
        tabs: &[
            Tab {
                label: "Library",
                icon: Some(icons::STORAGE),
            },
            Tab {
                label: "Activity",
                icon: Some(icons::ANIMATION),
            },
            Tab {
                label: "Settings",
                icon: Some(icons::SETTINGS),
            },
        ],
        selected: 2,
    }
    .draw(canvas, None);
    canvas.finish(dark)
}

pub fn sheet(canvas: &mut Canvas, dark: bool) -> Vec<u8> {
    canvas.begin();
    header(canvas, "Downloads", "Done");
    canvas.card(Rect::new(40, 325, 1184, 900));
    canvas.text(
        Rect::new(88, 374, 1088, 98),
        "Saved catalog",
        TextStyle::new(44).bold(),
    );

    // A grayscale sample picture stays the same in both themes and sits behind the modal.
    let picture = Rect::new(90, 490, 1084, 620);
    let width = picture.w as usize;
    let mut image = GrayImage::blank(picture.w as u32, picture.h as u32);
    for (index, pixel) in image.pixels.iter_mut().enumerate() {
        let x = index % width;
        let y = index / width;
        let stripe = (x.saturating_add(y / 2) / 60) % 3;
        *pixel = [85, 153, 204][stripe];
    }
    canvas.picture(picture, image);

    Sheet::centered(
        canvas.bounds(),
        "Clear saved catalog?",
        "Refresh the catalog to browse apps again.",
        "Clear",
    )
    .draw(canvas, None);
    canvas.finish(dark)
}

pub fn install_progress(canvas: &mut Canvas, dark: bool) -> Vec<u8> {
    canvas.begin();
    header(canvas, "Downloads", "Done");
    canvas.popup(Rect::new(200, 610, 864, 415));
    canvas.text(
        Rect::new(252, 671, 760, 70),
        "Installing app",
        TextStyle::new(50).bold(),
    );
    canvas.text(
        Rect::new(252, 760, 760, 65),
        "Checking the downloaded package",
        CAPTION,
    );
    progress(canvas, Rect::new(252, 856, 760, 44), 6);
    canvas.finish(dark)
}

pub fn keyboard(canvas: &mut Canvas, dark: bool) -> Vec<u8> {
    canvas.begin();
    header(canvas, "Text input", "Done");
    let mut editor = Editor::new(64);
    editor.text = "Write on a Kindle".chars().collect();
    editor.caret = 10;
    section_header(canvas, Rect::new(40, 230, 1184, 50), "Single line");
    canvas.single_line_field(Rect::new(40, 290, 1184, 100), &editor, "Enter text", false);
    editor.caret = editor.text.len();
    section_header(canvas, Rect::new(40, 450, 1184, 50), "Masked input");
    canvas.single_line_field(Rect::new(40, 510, 1184, 100), &editor, "Enter text", true);
    canvas.keyboard(Layer::Lower, "Done", None);
    canvas.finish(dark)
}

pub fn message(canvas: &mut Canvas, dark: bool) -> Vec<u8> {
    canvas.begin();
    full_screen_message(
        canvas,
        "Download complete",
        "Your app is ready to open from the library.",
    );
    canvas.finish(dark)
}

pub const PRESSED_ROW: Rect = Rect::new(ROWS.x, ROWS.y + 120, ROWS.w, 120);
