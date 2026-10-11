# Changelog

## v0.1.3 — 2026-10-05

### Changed

- Drawing a label or a hint no longer allocates on any frame, with the same picture, and a counting-allocator test holds that drawing allocates nothing.
- Widths and positions add with saturation, so a very wide label can't overflow them.
- The crate's description, tests and README speak to any ratatui app.
- `rust-version` is dropped, since no gate tests a minimum.

## v0.1.2 — 2026-10-04

### Changed

- `CYCLE` is one full turn of the glass, drain plus flip, derived from the frame timing.

## v0.1.1 — 2026-10-04

### Changed

- The crate is public, with Gamer Dad as its only author, and its repository points at its public home.
- The README shows the https source to pin.

## v0.1.0 — 2026-10-03

### Added

- The Braille hourglass spinner as a ratatui widget, its copy shimmering in the app's accent.
