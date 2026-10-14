# Notices

pito-hourglass's 13 hourglass frames (`FRAMES`: 7 drain, then 6 flip) are
copied verbatim from [basecamp/hey-cli](https://github.com/basecamp/hey-cli),
`internal/tui/loading.go`, and its drain and flip timing follows that file's
`hourglassFrameIndex`. They are used under hey-cli's MIT licence, reproduced
here in full:

```text
MIT License

Copyright (c) 2026 37signals, LLC

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

# README images

The images in `docs/images/` are this crate's own, under its MIT licence
like the code (© 2026 Cătălin Ilinca); the names and logos in them are
covered by TRADEMARKS.md, not by that licence. Each comes from the crate
itself:

- `demo.gif`: `examples/demo.rs`, recorded with prec (`render/terminal.toml`,
  shot `demo`) and made a GIF with pconv.
- `gallery.gif`: `examples/gallery.rs`, shot `gallery`, the same way.
- `fit.gif`: `examples/fit.rs`, shot `fit`, the same way.
- `frames.png`: `examples/frames.rs`, a prec still (shot `frames`).
- `draw-time.png`: drawn by pito-compare's `pcmp` from `draw-time.toml`,
  which `examples/bench.rs` printed on 2026-10-14.
