# Conventions for a PrimitiveOS app

Written for: anyone starting or changing a native app in this family (the Hello template is the starting point).

Every native app is built on the app kit (`primitive-app` and `primitive-ui` in the tools repo) and follows the look of
Primitive Launcher. The full standard, with the reasons, is in the `primitive-app-conventions` skill in the umbrella repo
(`doannt-kindle/.agents/skills/`). This file is the short version, with the places in this template that show each rule.

## 1. Build on the kit, not on the device

- Implement the kit's `App` trait and start the app with `primitive_app::run(app, &dir)`. The kit owns the e-ink
  screen, touch, the page buttons, rotation, sleep and wake, USB drive mode, storage, the system layer and the theme.
  See `crates/hello-kindle/src/main.rs`.
- An app has **no device code**: no framebuffer, no `/dev/input`, no accelerometer, no `lipc`, no power or sleep, no
  SIGUSR1 handler, no lock screen, no quick settings, no light or dark switch of its own.
- Draw with `primitive_ui::Canvas`: `header`, `screen_rect` (the 1264 × 1680 reference), the text styles `TITLE`,
  `BODY`, `CAPTION`, `button`, `keyboard`, `Rows`, `TabBar`, `Sheet`, `Pager`, `switch`, `progress`. See
  `crates/hello-desktop/src/gallery.rs`.
- Keep the layers apart: `<app>-core` (state and rules, no drawing), `<app>-render` (drawing and tap mapping),
  `<app>-kindle` (the entry point), `<app>-desktop` (a Mac driver for previews and tests).
- The library entry (`deploy/<Name>.sh`) has `# Wake: redraw` in its first ten lines and
  `[ "${PRIMITIVE_LAUNCHER:-0}" = 1 ] && trap '' USR1` before the app starts.

## 2. Tap feedback: every tappable control reverses (required)

A control reverses **black and white** while it is held: black becomes white, white becomes black. Not gray. The kit
controls do this by themselves. A control you draw yourself must do it too:

1. Register it while drawing: `let pressed = canvas.control_pressed(rect, false);`. The rectangle you register is the
   tap target, and it must be the rectangle your input code hit-tests.
2. Draw the pressed look in black and white, with the ink reversed too (text, icons, check marks).
3. For a toggle, reverse by state: `let filled = on != pressed;`. A checked toggle turns white while held, and an
   unchecked one turns black. The example is `custom_toggle` in `crates/hello-render/src/lib.rs`.
4. Never a gray pressed fill. A control that does nothing when tapped does not need the look.

Check it: `crates/hello-desktop/src/main.rs` holds the custom toggle and asserts that only its rectangle changed.
Add one such check for each control you draw yourself.

## 3. Dark mode and icons

- Never invert colours in the app and never add a light or dark switch: the kit draws the page in the system's theme.
  Pictures keep their own shades.
- An app icon is two files in `assets/icons/apps/`: `<name>.png` (black on white, 256 px) and `<name>-dark.png`
  (white on black). Make the light file, then run `python3 -I scripts/icon-forms.py <dir>` to make the dark file.

## 4. Tests

- `cargo test --workspace --locked`. The drawing tests need the Noto Sans fonts: set `KL_TEST_FONTS` to the launcher's
  `assets/fonts`. Without it they skip. Run the checks as CI does, with the variable **unset** when you do not have fonts.
- `cargo clippy --workspace --all-targets -- -D warnings` must pass.
- Every new screen gets a desktop preview, and each changed control a pressed-frame check.

## 5. Versions, changelog and release

- The version is the Cargo version. A release bumps the patch and has a `## <version>` entry at the top of
  `CHANGELOG.md`. Commit `Cargo.lock` with the bump (CI runs `--locked`, and a stale lock fails the release).
- The release steps (branch, merge into `main`, checks, tag `v<version>`, the Release workflow, the Store listing) are in
  the `kindle-release-pipeline` skill in the umbrella repo.

## 6. Store listing

Each release gets an entry in the catalog (`apps/primitive-<name>/app.json`). `tested_on` lists only the Kindle models
the version was really run on. Active listings use armhf only until armel is tested on a device.
