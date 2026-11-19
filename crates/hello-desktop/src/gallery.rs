use primitive_ui::{header, progress, switch, Canvas, Layer, Rect, TextStyle, CAPTION};

pub fn controls(canvas: &mut Canvas, dark: bool) -> Vec<u8> {
    canvas.begin();
    header(canvas, "Interface components", "Done");
    canvas.button(Rect::new(40, 195, 364, 100), "Primary", true, false);
    canvas.button(Rect::new(450, 195, 364, 100), "Secondary", false, false);
    canvas.button(Rect::new(860, 195, 364, 100), "Pressed", false, true);

    canvas.text(Rect::new(40, 347, 1184, 60), "SETTINGS", CAPTION.bold());
    canvas.card(Rect::new(40, 405, 1184, 236));
    canvas.text(
        Rect::new(72, 435, 900, 55),
        "Updates",
        TextStyle::new(38).bold(),
    );
    canvas.text(Rect::new(72, 485, 900, 55), "Check once a day", CAPTION);
    switch(canvas, Rect::new(1050, 428, 112, 64), true);
    canvas.fill(Rect::new(72, 520, 1120, 2), 200);
    canvas.text(
        Rect::new(72, 555, 900, 55),
        "Dark mode",
        TextStyle::new(38).bold(),
    );
    switch(canvas, Rect::new(1050, 548, 112, 64), false);

    canvas.popup(Rect::new(200, 692, 864, 415));
    canvas.text(
        Rect::new(252, 753, 760, 70),
        "Installing app",
        TextStyle::new(50).bold(),
    );
    canvas.text(
        Rect::new(252, 842, 760, 65),
        "Checking the downloaded package",
        CAPTION,
    );
    progress(canvas, Rect::new(252, 938, 760, 44), 6);
    canvas.finish(dark)
}

pub fn keyboard(canvas: &mut Canvas, dark: bool) -> Vec<u8> {
    canvas.begin();
    header(canvas, "Keyboard", "Done");
    canvas.card(Rect::new(40, 235, 1184, 130));
    canvas.text(
        Rect::new(69, 273, 1126, 65),
        "Write on a Kindle",
        TextStyle::new(43),
    );
    canvas.keyboard(Layer::Lower, "Done", None);
    canvas.finish(dark)
}
