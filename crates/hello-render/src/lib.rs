use std::io;
use std::path::PathBuf;

use hello_core::Hello;
use primitive_app::{App, Change};
use primitive_ui::{back_button, header, screen_rect, Canvas, Input, Rect, TextStyle, BLACK, CAPTION, PAPER};

pub struct View {
    pub hello: Hello,
    pub data: Option<PathBuf>,
}

impl App for View {
    fn draw(&self, canvas: &mut Canvas) {
        let width = canvas.bounds().w;
        header(canvas, "Hello", "Done");
        canvas.text(screen_rect(width, 40, 470, 1184, 140), "Hello, Kindle", TextStyle::new(78).bold().center());
        canvas.text(screen_rect(width, 40, 620, 1184, 70), &format!("Greetings sent: {}", self.hello.count()), CAPTION.center());
        canvas.button(screen_rect(width, 412, 795, 440, 100), "Say hello", true, false);
    }

    fn input(&mut self, input: Input) -> Change {
        match input {
            Input::Tap(x, y) if back_button(1264).contains(x, y) => Change::Exit,
            Input::Tap(x, y) if screen_rect(1264, 412, 795, 440, 100).contains(x, y) => {
                self.hello.greet();
                Change::Partial
            }
            _ => Change::None,
        }
    }

    fn save(&mut self) -> io::Result<()> {
        if let Some(path) = &self.data {
            primitive_app::save(path, self.hello.count().to_string().as_bytes())?;
        }
        Ok(())
    }
}

/// A control the kit does not draw for you: a custom toggle. It follows the tap-feedback rule (docs/CONVENTIONS.md §2):
/// it registers its rectangle as a tap target, reverses black and white while it is held, and inverts its state, so a
/// checked toggle turns white while it is held and an unchecked one turns black.
pub fn custom_toggle(canvas: &mut Canvas, rect: Rect, on: bool) {
    let pressed = canvas.control_pressed(rect, false);
    let filled = on != pressed;
    canvas.panel(rect, 22, 3, if filled { BLACK } else { PAPER });
    let ink = if filled { PAPER } else { BLACK };
    let top = (rect.h - canvas.px(45)).max(0) / 2;
    let label = if on { "On" } else { "Off" };
    canvas.text(
        Rect::new(rect.x, rect.y + top, rect.w, rect.h - top),
        label,
        TextStyle::new(36).bold().center().ink(ink),
    );
}
