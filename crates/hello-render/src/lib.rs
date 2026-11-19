use std::io;
use std::path::PathBuf;

use hello_core::Hello;
use primitive_app::{App, Change};
use primitive_ui::{back_button, header, screen_rect, Canvas, Input, TextStyle, CAPTION};

pub struct View {
    pub hello: Hello,
    pub data: Option<PathBuf>,
}

impl App for View {
    fn draw(&self, canvas: &mut Canvas) {
        let width = canvas.bounds().w;
        header(canvas, "Primitive Hello", "Done");
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
