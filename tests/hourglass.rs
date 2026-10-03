use std::time::Duration;

use pito_hourglass::{FRAMES, GLASS_ROWS, Hourglass, WIDTH, band_at, frame_at};
use ratatui::{
    Terminal,
    backend::TestBackend,
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::Widget,
};
use unicode_width::UnicodeWidthStr;

const GLASS: Style = Style::new().fg(Color::Yellow);
const ACCENT: Style = Style::new().fg(Color::Magenta);
const MUTED: Style = Style::new().fg(Color::DarkGray).add_modifier(Modifier::DIM);
const LABEL: &str = "Loading...";
const HINT: &str = "esc stops waiting";

fn ms(n: u64) -> Duration {
    Duration::from_millis(n)
}

fn styled(elapsed: Duration) -> Hourglass<'static> {
    Hourglass::new(elapsed)
        .glass(GLASS)
        .accent(ACCENT)
        .muted(MUTED)
}

fn hourglass(elapsed: Duration) -> Hourglass<'static> {
    styled(elapsed).label(LABEL).hint(Some(HINT))
}

struct Drawn {
    text: Vec<String>,
    marks: Vec<String>,
}

impl Drawn {
    fn blank(&self) -> bool {
        self.text.iter().all(String::is_empty) && self.marks.iter().all(String::is_empty)
    }
}

fn draw(widget: Hourglass<'_>, width: u16, height: u16, area: Rect) -> Drawn {
    let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
    terminal
        .draw(|frame| frame.render_widget(widget, area))
        .unwrap();
    read(terminal.backend().buffer())
}

fn whole(widget: Hourglass<'_>, width: u16, height: u16) -> Drawn {
    draw(widget, width, height, Rect::new(0, 0, width, height))
}

fn mark(fg: Color, modifier: Modifier) -> char {
    match (fg, modifier) {
        (Color::Reset, m) if m.is_empty() => ' ',
        (Color::Yellow, m) if m.is_empty() => 'g',
        (Color::Magenta, Modifier::BOLD) => 'A',
        (Color::Magenta, m) if m.is_empty() => 'a',
        (Color::DarkGray, Modifier::DIM) => 'm',
        _ => '?',
    }
}

fn read(buffer: &Buffer) -> Drawn {
    let width = usize::from(buffer.area.width).max(1);
    let mut text = Vec::new();
    let mut marks = Vec::new();
    for row in buffer.content.chunks(width) {
        let mut line = String::new();
        let mut hidden = 0;
        for cell in row {
            if hidden > 0 {
                hidden -= 1;
                continue;
            }
            line.push_str(cell.symbol());
            hidden = cell.symbol().width().saturating_sub(1);
        }
        let marked: String = row
            .iter()
            .map(|cell| mark(cell.fg, cell.modifier))
            .collect();
        text.push(line.trim_end().to_string());
        marks.push(marked.trim_end().to_string());
    }
    Drawn { text, marks }
}

fn glass_marks() -> String {
    format!("     {}", "g".repeat(11))
}

#[test]
fn every_frame_is_eleven_cells_wide_and_six_rows_high() {
    assert_eq!(FRAMES.len(), 13);
    assert_eq!(WIDTH, 11);
    assert_eq!(GLASS_ROWS, 6);
    for frame in FRAMES {
        assert_eq!(frame.len(), 6);
        for line in frame {
            assert_eq!(line.width(), 11, "{line}");
            assert_eq!(line.chars().count(), 11, "{line}");
        }
    }
    assert_eq!(FRAMES[0], FRAMES[12]);
}

#[test]
fn the_frame_follows_the_elapsed_time() {
    let expected = [
        (0, 0),
        (428, 0),
        (429, 1),
        (2999, 6),
        (3000, 7),
        (3199, 7),
        (3200, 8),
        (4199, 12),
        (4200, 0),
        (4200 + 3000, 7),
        (8400 + 429, 1),
    ];
    for (at, frame) in expected {
        assert_eq!(frame_at(ms(at)), frame, "at {at} ms");
    }
}

#[test]
fn every_frame_shows_for_its_share_of_the_cycle() {
    let mut seen = [0u32; 13];
    for at in 0..4200 {
        seen[frame_at(ms(at))] += 1;
    }
    assert_eq!(&seen[..7], [429, 429, 428, 429, 428, 429, 428]);
    assert_eq!(&seen[7..], [200; 6]);
}

#[test]
fn the_band_follows_the_elapsed_time() {
    let expected = [
        (0, None),
        (200, Some(0)),
        (600, Some(5)),
        (1199, None),
        (1200, None),
        (1499, None),
        (1500, None),
        (1500 + 200, Some(0)),
        (3000 + 600, Some(5)),
    ];
    for (at, band) in expected {
        assert_eq!(band_at(ms(at), 10), band, "at {at} ms");
    }
}

#[test]
fn the_band_enters_at_the_first_character_and_leaves_after_the_last() {
    assert_eq!(band_at(ms(171), 10), None);
    assert_eq!(band_at(ms(172), 10), Some(0));
    assert_eq!(band_at(ms(257), 10), Some(0));
    assert_eq!(band_at(ms(258), 10), Some(1));
    assert_eq!(band_at(ms(942), 10), Some(8));
    assert_eq!(band_at(ms(943), 10), Some(9));
    assert_eq!(band_at(ms(1028), 10), Some(9));
    assert_eq!(band_at(ms(1029), 10), None);
    let visible: Vec<usize> = (0..1500).filter_map(|at| band_at(ms(at), 10)).collect();
    assert_eq!(visible.first(), Some(&0));
    assert_eq!(visible.last(), Some(&9));
    assert!(
        visible
            .windows(2)
            .all(|pair| pair[1] == pair[0] || pair[1] == pair[0] + 1)
    );
}

#[test]
fn an_empty_label_has_no_band() {
    assert!((0..1500).all(|at| band_at(ms(at), 0).is_none()));
    assert_eq!(band_at(ms(479), 1), None);
    assert_eq!(band_at(ms(480), 1), Some(0));
}

#[test]
fn the_full_layout_at_the_start_of_a_cycle() {
    let drawn = whole(hourglass(ms(0)), 21, 10);
    assert_eq!(
        drawn.text,
        [
            "",
            "        ╭───╮",
            "        │⣿⣿⣿│",
            "        ╰╮⣿╭╯",
            "        ╭╯ ╰╮",
            "        │   │",
            "        ╰───╯",
            "     Loading...",
            "  esc stops waiting",
            "",
        ]
    );
    let glass = glass_marks();
    let glass = glass.as_str();
    assert_eq!(
        drawn.marks,
        [
            "",
            glass,
            glass,
            glass,
            glass,
            glass,
            glass,
            "     mmmmmmmmmm",
            "  mmmmmmmmmmmmmmmmm",
            "",
        ]
    );
    assert_eq!(band_at(ms(0), LABEL.len()), None);
}

#[test]
fn the_full_layout_while_the_glass_flips() {
    let drawn = whole(hourglass(ms(3300)), 21, 10);
    assert_eq!(frame_at(ms(3300)), 8);
    assert_eq!(
        drawn.text,
        [
            "",
            "     ╭─",
            "     │ ──╮",
            "     ╰─   ⣠╭──",
            "       ──╯ ⣾⣿⣿─╮",
            "           ╰──⣿│",
            "              ─╯",
            "     Loading...",
            "  esc stops waiting",
            "",
        ]
    );
    let glass = glass_marks();
    let glass = glass.as_str();
    assert_eq!(
        drawn.marks,
        [
            "",
            glass,
            glass,
            glass,
            glass,
            glass,
            glass,
            "     aAammmmmmm",
            "  mmmmmmmmmmmmmmmmm",
            "",
        ]
    );
    assert_eq!(band_at(ms(3300), LABEL.len()), Some(1));
}

#[test]
fn the_full_layout_mid_sweep() {
    let drawn = whole(hourglass(ms(600)), 21, 10);
    assert_eq!(drawn.text[1], "        ╭───╮");
    assert_eq!(drawn.text[4], "        ╭╯⡇╰╮");
    assert_eq!(drawn.marks[7], "     mmmmaAammm");
    assert_eq!(drawn.marks[8], "  mmmmmmmmmmmmmmmmm");
}

#[test]
fn the_glass_shows_the_frame_for_the_elapsed_time() {
    for at in (0..4200).step_by(50) {
        let drawn = whole(hourglass(ms(at)), 21, 10);
        let frame = FRAMES[frame_at(ms(at))];
        for (row, line) in frame.iter().enumerate() {
            let expected = format!("     {line}");
            assert_eq!(drawn.text[row + 1], expected.trim_end(), "at {at} ms");
        }
    }
}

#[test]
fn the_accent_follows_band_at_and_the_hint_never_shimmers() {
    for at in (0..3000).step_by(10) {
        let drawn = whole(hourglass(ms(at)), 21, 10);
        let label = format!("{:<15}", drawn.marks[7]);
        let cells: Vec<char> = label.chars().skip(5).take(10).collect();
        let bold: Vec<usize> = (0..10).filter(|i| cells[*i] == 'A').collect();
        assert_eq!(
            bold,
            band_at(ms(at), 10).into_iter().collect::<Vec<_>>(),
            "at {at} ms"
        );
        if let Some(centre) = band_at(ms(at), 10) {
            for near in [centre.wrapping_sub(1), centre + 1] {
                if near < 10 {
                    assert_eq!(cells[near], 'a', "at {at} ms");
                }
            }
        }
        let lit = cells.iter().filter(|cell| **cell != 'm').count();
        assert!(lit <= 3, "at {at} ms: {label}");
        assert_eq!(drawn.marks[8], "  mmmmmmmmmmmmmmmmm", "at {at} ms");
    }
}

#[test]
fn the_band_clips_at_both_ends_of_the_label() {
    let entering = whole(hourglass(ms(100)), 21, 10);
    assert_eq!(entering.marks[7], "     ammmmmmmmm");
    let leaving = whole(hourglass(ms(1050)), 21, 10);
    assert_eq!(leaving.marks[7], "     mmmmmmmmma");
    for at in [1200, 1350, 1499] {
        let paused = whole(hourglass(ms(at)), 21, 10);
        assert_eq!(paused.marks[7], "     mmmmmmmmmm", "at {at} ms");
    }
}

#[test]
fn the_block_centres_in_an_area_away_from_the_corner() {
    let corner = whole(hourglass(ms(3300)), 21, 10);
    let away = draw(hourglass(ms(3300)), 25, 12, Rect::new(2, 1, 21, 10));
    assert_eq!(away.text[0], "");
    assert_eq!(away.text[11], "");
    for row in 0..10 {
        let shift = |line: &String| {
            if line.is_empty() {
                String::new()
            } else {
                format!("  {line}")
            }
        };
        assert_eq!(away.text[row + 1], shift(&corner.text[row]));
        assert_eq!(away.marks[row + 1], shift(&corner.marks[row]));
    }
}

#[test]
fn without_a_hint_seven_rows_hold_the_glass() {
    let drawn = whole(styled(ms(0)).label(LABEL), 30, 7);
    assert_eq!(drawn.text[0], "            ╭───╮");
    assert_eq!(drawn.text[5], "            ╰───╯");
    assert_eq!(drawn.text[6], "          Loading...");
    let short = whole(styled(ms(0)).label(LABEL), 30, 6);
    assert!(!short.text.concat().contains('╭'));
    assert_eq!(short.text[2], "          Loading...");
    let empty = whole(styled(ms(0)).label(LABEL).hint(Some("")), 30, 7);
    assert_eq!(empty.text, drawn.text);
}

#[test]
fn a_narrow_area_drops_the_glass_and_keeps_the_shimmer() {
    let drawn = whole(hourglass(ms(600)), 10, 4);
    assert_eq!(drawn.text, ["", "Loading...", "esc stops", ""]);
    assert_eq!(drawn.marks, ["", "mmmmaAammm", "mmmmmmmmmm", ""]);
    let tall = whole(hourglass(ms(3300)), 10, 30);
    assert!(!tall.text.concat().contains('╭'));
    assert_eq!(tall.text[14], "Loading...");
    assert_eq!(tall.marks[14], "aAammmmmmm");
    assert_eq!(tall.text[15], "esc stops");
}

#[test]
fn a_short_area_drops_the_glass() {
    let drawn = whole(hourglass(ms(3300)), 30, 7);
    assert_eq!(
        drawn.text,
        [
            "",
            "",
            "          Loading...",
            "      esc stops waiting",
            "",
            "",
            "",
        ]
    );
    assert_eq!(drawn.marks[2], "          aAammmmmmm");
    assert_eq!(drawn.marks[3], "      mmmmmmmmmmmmmmmmm");
    let fits = whole(hourglass(ms(3300)), 30, 8);
    assert!(fits.text.concat().contains('╭'));
}

#[test]
fn one_row_holds_the_label_alone() {
    let drawn = whole(hourglass(ms(600)), 30, 1);
    assert_eq!(drawn.text, ["          Loading..."]);
    assert_eq!(drawn.marks, ["          mmmmaAammm"]);
    let inside = draw(hourglass(ms(600)), 30, 3, Rect::new(0, 0, 30, 1));
    assert_eq!(inside.text, ["          Loading...", "", ""]);
    assert_eq!(inside.marks, ["          mmmmaAammm", "", ""]);
}

#[test]
fn a_zero_area_draws_nothing() {
    for area in [
        Rect::new(0, 0, 0, 0),
        Rect::new(0, 0, 0, 4),
        Rect::new(0, 0, 10, 0),
        Rect::new(3, 2, 0, 0),
    ] {
        assert!(draw(hourglass(ms(600)), 10, 4, area).blank(), "{area:?}");
    }
}

#[test]
fn a_long_label_is_cut_at_a_character() {
    let long = "Waiting for the estate board";
    let drawn = whole(styled(ms(600)).label(long), 12, 1);
    assert_eq!(drawn.text, ["Waiting for"]);
    assert_eq!(drawn.marks, ["mmmmmaAammmm"]);
    assert_eq!(band_at(ms(600), 12), Some(6));
    let glass = whole(styled(ms(0)).label(long).hint(Some(HINT)), 11, 8);
    assert_eq!(glass.text[0], "   ╭───╮");
    assert_eq!(glass.text[6], "Waiting for");
    assert_eq!(glass.text[7], "esc stops w");
}

#[test]
fn a_wide_glyph_is_never_split() {
    let label = "日本語のラベル";
    assert_eq!(whole(styled(ms(0)).label(label), 5, 1).text, ["日本"]);
    assert_eq!(whole(styled(ms(0)).label(label), 6, 1).text, ["日本語"]);
    assert_eq!(whole(styled(ms(0)).label(label), 3, 1).text, ["日"]);
    assert!(whole(styled(ms(0)).label(label), 1, 1).blank());
}

#[test]
fn a_label_with_diacritics_and_an_ellipsis() {
    let label = "Se încarcă datele…";
    assert_eq!(label.width(), 18);
    let drawn = whole(styled(ms(600)).label(label).hint(Some(HINT)), 21, 10);
    assert_eq!(drawn.text[1], "        ╭───╮");
    assert_eq!(drawn.text[7], " Se încarcă datele…");
    assert_eq!(drawn.marks[7], " mmmmmmmmaAammmmmmm");
    assert_eq!(band_at(ms(600), 18), Some(9));
    let short = whole(styled(ms(600)).label(label), 8, 1);
    assert_eq!(short.text, ["Se încar"]);
}

#[test]
fn a_combining_mark_stays_with_its_letter() {
    let label = "As\u{326}teptăm răspunsul…";
    assert_eq!(label.width(), 19);
    assert_eq!(whole(styled(ms(0)).label(label), 1, 1).text, ["A"]);
    assert_eq!(whole(styled(ms(0)).label(label), 2, 1).text, ["As\u{326}"]);
    assert_eq!(
        whole(styled(ms(0)).label(label), 30, 1).text,
        ["     As\u{326}teptăm răspunsul…"]
    );
    let at = (0..1200)
        .find(|at| band_at(ms(*at), 19) == Some(1))
        .unwrap();
    let drawn = whole(styled(ms(at)).label(label), 19, 1);
    assert_eq!(drawn.marks, [format!("aAa{}", "m".repeat(16))]);
}

#[test]
fn the_defaults_are_the_hey_copy_with_no_hint() {
    let drawn = whole(Hourglass::new(ms(0)), 30, 9);
    assert_eq!(drawn.text[1], "            ╭───╮");
    assert_eq!(drawn.text[7], "          Loading...");
    assert_eq!(drawn.text[8], "");
    let by_reference = {
        let mut terminal = Terminal::new(TestBackend::new(30, 9)).unwrap();
        let widget = Hourglass::new(ms(0));
        terminal
            .draw(|frame| {
                let area = frame.area();
                (&widget).render(area, frame.buffer_mut());
            })
            .unwrap();
        read(terminal.backend().buffer())
    };
    assert_eq!(by_reference.text, drawn.text);
}
