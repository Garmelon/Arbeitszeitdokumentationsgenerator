# Arbeitszeitdokumentationsgenerator

This repository contains tools to make filling out KIT time sheets easier. It is
inspired by the [SDQ TimeSheetGenerator][tsg] and a friend's very useful wrapper
website that spits out PDFs directly.

[tsg]: https://github.com/kit-sdq/TimeSheetGenerator

## Typst template

The centerpiece of the repository is a [typst][typst] template that generates
and validates time sheets. It aims to mirror the look of the official form as
closely as possible. For usage information, see [its docs][tdocs].

[typst]: https://github.com/typst/typst
[tdocs]: kit_timesheet.md

## Rust web server

In case people don't want to use the typst template directly, this repo contains
a small web server. It provides a web UI that lets people generate PDFs directly
from their browser.

In theory, you could also compile the code to WASM and generate the time sheets
directly in the browser, but that involves more JS than I'm willing to put up
with right now.
