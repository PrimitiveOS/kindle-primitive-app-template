# Primitive app template

Primitive Hello shows how a native app shares the launcher’s drawing, keyboard, input, rotation and sleep handling.
Run `cargo test --offline --workspace` and `cargo run --offline -p hello-desktop` from this directory with
`doannt-kindle-tools` next to it. The desktop target writes light and dark previews in `build/previews`.

The four crates hold app logic, drawing and hit testing, the Kindle entry point, and the host preview. The library
script copies the program to temporary storage. Run `scripts/stage.sh TARGET` after building the Kindle executable, and
package the stage with `doannt-kindle-build/scripts/package.py`.

Source is PolyForm Noncommercial 1.0.0. Documentation is CC BY-NC-SA 4.0.
