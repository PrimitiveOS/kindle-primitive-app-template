#!/usr/bin/env python3
"""Create a local PrimitiveOS app repository from the Hello workspace."""

import argparse
import os
import re
import subprocess
from pathlib import Path
from shutil import copyfile, copymode

TEMPLATE = Path(__file__).resolve().parents[1]
SKIP = {".git", "build", "dist", "target"}
REPLACE = (
    ("kindle-primitive-app-template", "kindle-primitive-{slug}"),
    ("Hello, Kindle", "{title}"),
    ('"Hello"', '"{title}"'),
    ("# Name: Hello", "# Name: {title}"),
    ("Hello.sh", "{symbol}.sh"),
    ("Hello", "{symbol}"),
    ("hello", "{slug}"),
)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("slug", help="app folder and Rust crate prefix")
    parser.add_argument("title", help="app display name")
    options = parser.parse_args()
    slug, title = options.slug, options.title.strip()
    if not re.fullmatch(r"[a-z][a-z0-9]*", slug) or not re.fullmatch(r"[A-Za-z][A-Za-z0-9 ]*", title):
        parser.error("use a lowercase alphanumeric slug and a plain title")
    target = TEMPLATE.parent / f"doannt-kindle-{slug}"
    if target.exists():
        parser.error(f"{target} already exists")
    symbol = slug.title()
    replacements = {
        pattern: replacement.format(slug=slug, title=title, symbol=symbol)
        for pattern, replacement in REPLACE
    }
    pattern = re.compile("|".join(map(re.escape, replacements)))
    target.mkdir()
    for root, dirs, names in os.walk(TEMPLATE):
        dirs[:] = sorted(set(dirs) - SKIP)
        for name in sorted(names):
            source = Path(root) / name
            relative = source.relative_to(TEMPLATE)
            if relative.as_posix() in ("scripts/new-app.py", "Cargo.lock"):
                continue
            dest = target / str(relative).replace("Hello", symbol).replace("hello", slug)
            dest.parent.mkdir(parents=True, exist_ok=True)
            if relative.suffix == ".png" or relative.parts[0] in ("LICENSE", "LICENSE-DOCS"):
                copyfile(source, dest)
            else:
                content = pattern.sub(
                    lambda match: title if relative.suffix == ".md" and match[0] == "Hello" else replacements[match[0]],
                    source.read_text(),
                )
                dest.write_text(content)
            copymode(source, dest)
    subprocess.run(["git", "init", "-b", f"feature/{slug}"], cwd=target, check=True)
    print(target)


if __name__ == "__main__":
    main()
