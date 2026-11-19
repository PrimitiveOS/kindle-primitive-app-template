#!/bin/sh
# Name: Primitive Hello
# Author: Primitive AI
# DontUseFBInk

app=/mnt/us/hello/bin/hello-kindle
[ -x "$app" ] || exit 1
work=$(mktemp -d /var/tmp/primitive-hello.XXXXXX) || exit 1
trap 'rm -rf "$work"' EXIT
cp "$app" "$work/run" && chmod 755 "$work/run" || exit 1
cd / || exit 1
"$work/run"
