# Primitive app template

Primitive Hello shows how a native app shares the launcher’s drawing, keyboard, input, rotation and sleep handling.
Run `cargo test --offline --workspace` and `cargo run --offline -p hello-desktop` from this directory with
`doannt-kindle-tools` next to it. The desktop target writes previews and a component gallery in
`build/previews`. It renders grouped rows, switches, tab and page navigation, the launcher status bar, a modal sheet,
progress blocks, the keyboard and a full-screen message. The gallery includes light, dark and upside-down frames and
checks that a pressed row changes only its own refresh rectangle. Primitive Hello also exercises sleep, wake, and
exit using the native lifecycle and verifies its saved count on disk.

The four crates hold app logic, drawing and hit testing, the Kindle entry point, and the host preview. The library
script copies the program to temporary storage. Run `scripts/stage.sh TARGET` after building the Kindle executable, and
package the stage with `doannt-kindle-build/scripts/package.py`.

Native apps inherit display preferences from Launcher. To override them for one app, add a `settings.txt` file in
that app's folder on user storage. Supported values are `dark_mode=on|off`, `auto_rotation=on|off`, and
`rotation=upright|upside-down`. The rotation value is used as the fixed orientation when auto-rotation is off.

Source is PolyForm Noncommercial 1.0.0. Documentation is CC BY-NC-SA 4.0.
