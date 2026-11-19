//! The editor's fonts (`replace-editor-fonts`).
//!
//! - **Inter** (OFL 1.1) sets the whole interface.
//! - **IBM Plex Sans SemiBold** (OFL 1.1) sets the brand block in the header.
//! - **Corrosion Icons**, a small font built here, holds the four triangles egui
//!   draws as text and Inter lacks. The `⏵` on the preset browser's folder
//!   submenus is one of them.
//!
//! They replace egui's default fonts (Ubuntu Light, Noto Emoji, Hack and an
//! emoji icon font), which the build leaves out: `vendor/egui-baseview` drops
//! `default_fonts` from its default features, and no other crate enables it.
//! The files and their licenses are in `assets/fonts/`.

#[cfg(feature = "gui")]
use nih_plug_egui::egui;
use std::sync::Arc;

use egui::{FontData, FontDefinitions, FontFamily, FontTweak};

/// Inter 4.1, Regular (`assets/fonts/Inter-Regular.ttf`).
const INTER: &[u8] = include_bytes!("../../assets/fonts/Inter-Regular.ttf");
/// IBM Plex Sans 1.1.0, SemiBold (`assets/fonts/IBMPlexSans-SemiBold.ttf`).
const PLEX_SANS: &[u8] = include_bytes!("../../assets/fonts/IBMPlexSans-SemiBold.ttf");

const INTER_NAME: &str = "Inter";
const PLEX_NAME: &str = "IBM Plex Sans";
const ICONS_NAME: &str = "Corrosion Icons";

/// The brand block's font family (IBM Plex Sans).
pub(super) fn brand_family() -> FontFamily {
    FontFamily::Name("brand".into())
}

/// Cap height of Ubuntu Light, the editor's previous font, per em.
const PREVIOUS_CAP_HEIGHT: f32 = 693.0 / 1000.0;
/// Cap height of Inter 4.1 per em (its OS/2 table: 1490 / 2048).
const INTER_CAP_HEIGHT: f32 = 1490.0 / 2048.0;
/// Cap height of IBM Plex Sans per em (its OS/2 table: 698 / 1000).
const PLEX_CAP_HEIGHT: f32 = 698.0 / 1000.0;

/// Every editor font size is multiplied by this, so Inter's capitals are as
/// tall as the previous font's at the sizes the layout was tuned for. It also
/// keeps the row height (Inter's 1.21 em × 0.9525 = 1.153 em, against Ubuntu
/// Light's 1.149 em), so no panel grows.
pub(super) const UI_EM_SCALE: f32 = PREVIOUS_CAP_HEIGHT / INTER_CAP_HEIGHT;

/// IBM Plex Sans's glyphs are scaled by this, on top of [`UI_EM_SCALE`], so
/// the brand block's capitals keep the previous font's height too.
const PLEX_GLYPH_SCALE: f32 = PREVIOUS_CAP_HEIGHT / (PLEX_CAP_HEIGHT * UI_EM_SCALE);

/// The editor's font definitions: Inter (then the icons) for the
/// proportional and monospace families, and IBM Plex Sans (then Inter and
/// the icons) for the brand family.
pub(super) fn definitions() -> FontDefinitions {
    let mut fonts = FontDefinitions::empty();
    fonts
        .font_data
        .insert(INTER_NAME.into(), Arc::new(FontData::from_static(INTER)));
    fonts.font_data.insert(
        PLEX_NAME.into(),
        Arc::new(FontData::from_static(PLEX_SANS).tweak(FontTweak {
            scale: PLEX_GLYPH_SCALE,
            ..FontTweak::default()
        })),
    );
    fonts.font_data.insert(
        ICONS_NAME.into(),
        Arc::new(FontData::from_owned(icon_font())),
    );
    let interface = vec![INTER_NAME.to_string(), ICONS_NAME.to_string()];
    fonts
        .families
        .insert(FontFamily::Proportional, interface.clone());
    fonts.families.insert(FontFamily::Monospace, interface);
    fonts.families.insert(
        brand_family(),
        vec![
            PLEX_NAME.to_string(),
            INTER_NAME.to_string(),
            ICONS_NAME.to_string(),
        ],
    );
    fonts
}

/// Give `ctx` the editor's fonts. Called once when the editor's context is
/// made (the plugin window opening, the web app starting), not per frame.
pub fn install(ctx: &egui::Context) {
    ctx.set_fonts(definitions());
}

// ---------------------------------------------------------------------------
// Corrosion Icons: a TrueType font built in code
// ---------------------------------------------------------------------------

/// The icon font's em, ascender and descender: Inter's, so a fallback glyph
/// sits on the same baseline and centre line as the Inter text around it.
const ICON_UNITS_PER_EM: u16 = 2048;
const ICON_ASCENDER: i16 = 1984;
const ICON_DESCENDER: i16 = -494;
/// Advance of every icon glyph, in font units.
const ICON_ADVANCE: u16 = 1400;

/// The icon glyphs: a code point and a filled triangle, its corners in font
/// units, clockwise (TrueType's outer-contour direction). Centred on y = 640,
/// around the middle of Inter's x-height, and about as large as the arrow of
/// egui's former icon font.
const ICON_GLYPHS: [(char, [(i16, i16); 3]); 4] = [
    // ⏴ pointing left
    ('\u{23F4}', [(1150, 90), (250, 640), (1150, 1190)]),
    // ⏵ pointing right (egui's submenu arrow)
    ('\u{23F5}', [(250, 90), (250, 1190), (1150, 640)]),
    // ⏶ pointing up
    ('\u{23F6}', [(150, 190), (700, 1090), (1250, 190)]),
    // ⏷ pointing down
    ('\u{23F7}', [(150, 1090), (1250, 1090), (700, 190)]),
];

/// The Corrosion Icons font as TrueType bytes: `.notdef` (empty) and the four
/// triangles of [`ICON_GLYPHS`], mapped from their code points. Only the
/// tables a TrueType reader needs to draw them: `cmap`, `glyf`, `head`,
/// `hhea`, `hmtx`, `loca`, `maxp` and `post`.
pub(super) fn icon_font() -> Vec<u8> {
    let num_glyphs = ICON_GLYPHS.len() as u16 + 1;

    // glyf and loca (short offsets: each glyph's length is even).
    let mut glyf = Vec::new();
    let mut loca = vec![0u16]; // .notdef has no outline
    loca.push(0);
    for (_, corners) in ICON_GLYPHS {
        let xs = corners.map(|c| c.0);
        let ys = corners.map(|c| c.1);
        let x_min = *xs.iter().min().unwrap();
        let y_min = *ys.iter().min().unwrap();
        let x_max = *xs.iter().max().unwrap();
        let y_max = *ys.iter().max().unwrap();
        push_i16(&mut glyf, 1); // one contour
        for v in [x_min, y_min, x_max, y_max] {
            push_i16(&mut glyf, v);
        }
        push_u16(&mut glyf, 2); // last point of the contour
        push_u16(&mut glyf, 0); // no instructions
        glyf.extend([0x01u8; 3]); // three on-curve points, 16-bit deltas
        let mut previous = 0i16;
        for x in xs {
            push_i16(&mut glyf, x - previous);
            previous = x;
        }
        previous = 0;
        for y in ys {
            push_i16(&mut glyf, y - previous);
            previous = y;
        }
        if glyf.len() % 2 == 1 {
            glyf.push(0);
        }
        loca.push((glyf.len() / 2) as u16);
    }
    let loca: Vec<u8> = loca.iter().flat_map(|v| v.to_be_bytes()).collect();

    // cmap: one format 4 subtable (Windows, Unicode BMP). The four code
    // points are consecutive, so one segment plus the closing 0xFFFF.
    let first = ICON_GLYPHS[0].0 as u16;
    let last = ICON_GLYPHS[ICON_GLYPHS.len() - 1].0 as u16;
    debug_assert_eq!(usize::from(last - first) + 1, ICON_GLYPHS.len());
    let mut format4 = Vec::new();
    let seg_count = 2u16;
    push_u16(&mut format4, 4); // format
    push_u16(&mut format4, 16 + 8 * seg_count); // length
    push_u16(&mut format4, 0); // language
    push_u16(&mut format4, seg_count * 2);
    push_u16(&mut format4, 4); // searchRange = 2 × 2^floor(log2(segCount))
    push_u16(&mut format4, 1); // entrySelector
    push_u16(&mut format4, 0); // rangeShift
    for end in [last, 0xFFFF] {
        push_u16(&mut format4, end);
    }
    push_u16(&mut format4, 0); // reservedPad
    for start in [first, 0xFFFF] {
        push_u16(&mut format4, start);
    }
    for delta in [1u16.wrapping_sub(first), 1] {
        push_u16(&mut format4, delta);
    }
    for _ in 0..seg_count {
        push_u16(&mut format4, 0); // idRangeOffset
    }
    let mut cmap = Vec::new();
    push_u16(&mut cmap, 0); // version
    push_u16(&mut cmap, 1); // one encoding record
    push_u16(&mut cmap, 3); // Windows
    push_u16(&mut cmap, 1); // Unicode BMP
    push_u32(&mut cmap, 12); // subtable offset
    cmap.extend(format4);

    let all_x = ICON_GLYPHS.iter().flat_map(|(_, c)| c.map(|p| p.0));
    let all_y = ICON_GLYPHS.iter().flat_map(|(_, c)| c.map(|p| p.1));
    let (x_min, x_max) = (all_x.clone().min().unwrap(), all_x.max().unwrap());
    let (y_min, y_max) = (all_y.clone().min().unwrap(), all_y.max().unwrap());

    let mut head = Vec::new();
    push_u32(&mut head, 0x0001_0000); // version 1.0
    push_u32(&mut head, 0x0001_0000); // fontRevision 1.0
    push_u32(&mut head, 0); // checkSumAdjustment, set below
    push_u32(&mut head, 0x5F0F_3CF5); // magicNumber
    push_u16(&mut head, 0x000B); // flags: baseline and LSB at 0, integer ppem
    push_u16(&mut head, ICON_UNITS_PER_EM);
    head.extend([0u8; 16]); // created, modified
    for v in [x_min, y_min, x_max, y_max] {
        push_i16(&mut head, v);
    }
    push_u16(&mut head, 0); // macStyle
    push_u16(&mut head, 8); // lowestRecPPEM
    push_i16(&mut head, 2); // fontDirectionHint
    push_i16(&mut head, 0); // indexToLocFormat: short
    push_i16(&mut head, 0); // glyphDataFormat

    let mut hhea = Vec::new();
    push_u32(&mut hhea, 0x0001_0000);
    push_i16(&mut hhea, ICON_ASCENDER);
    push_i16(&mut hhea, ICON_DESCENDER);
    push_i16(&mut hhea, 0); // lineGap
    push_u16(&mut hhea, ICON_ADVANCE); // advanceWidthMax
    push_i16(&mut hhea, 0); // minLeftSideBearing
    push_i16(&mut hhea, 0); // minRightSideBearing
    push_i16(&mut hhea, x_max); // xMaxExtent
    push_i16(&mut hhea, 1); // caretSlopeRise
    push_i16(&mut hhea, 0); // caretSlopeRun
    hhea.extend([0u8; 10]); // caretOffset and four reserved
    push_i16(&mut hhea, 0); // metricDataFormat
    push_u16(&mut hhea, num_glyphs); // numberOfHMetrics

    let mut hmtx = Vec::new();
    push_u16(&mut hmtx, ICON_ADVANCE); // .notdef
    push_i16(&mut hmtx, 0);
    for (_, corners) in ICON_GLYPHS {
        push_u16(&mut hmtx, ICON_ADVANCE);
        push_i16(&mut hmtx, corners.iter().map(|c| c.0).min().unwrap());
    }

    let mut maxp = Vec::new();
    push_u32(&mut maxp, 0x0001_0000);
    push_u16(&mut maxp, num_glyphs);
    push_u16(&mut maxp, 3); // maxPoints
    push_u16(&mut maxp, 1); // maxContours
    maxp.extend([0u8; 4]); // maxCompositePoints, maxCompositeContours
    push_u16(&mut maxp, 2); // maxZones
    maxp.extend([0u8; 16]); // the remaining eight limits, all 0

    let mut post = Vec::new();
    push_u32(&mut post, 0x0003_0000); // version 3: no glyph names
    post.extend([0u8; 28]);

    let tables: [(&[u8; 4], Vec<u8>); 8] = [
        (b"cmap", cmap),
        (b"glyf", glyf),
        (b"head", head),
        (b"hhea", hhea),
        (b"hmtx", hmtx),
        (b"loca", loca),
        (b"maxp", maxp),
        (b"post", post),
    ];
    let num_tables = tables.len() as u16;
    let mut font = Vec::new();
    push_u32(&mut font, 0x0001_0000); // TrueType outlines
    push_u16(&mut font, num_tables);
    push_u16(&mut font, 128); // searchRange = 16 × 2^floor(log2(8))
    push_u16(&mut font, 3); // entrySelector
    push_u16(&mut font, 0); // rangeShift
    let mut offset = 12 + 16 * tables.len();
    let mut body = Vec::new();
    let mut head_offset = 0;
    for (tag, data) in &tables {
        if *tag == b"head" {
            head_offset = offset;
        }
        font.extend_from_slice(*tag);
        push_u32(&mut font, checksum(data));
        push_u32(&mut font, offset as u32);
        push_u32(&mut font, data.len() as u32);
        body.extend_from_slice(data);
        while body.len() % 4 != 0 {
            body.push(0);
        }
        offset = 12 + 16 * tables.len() + body.len();
    }
    font.extend(body);
    let adjustment = 0xB1B0_AFBAu32.wrapping_sub(checksum(&font));
    font[head_offset + 8..head_offset + 12].copy_from_slice(&adjustment.to_be_bytes());
    font
}

fn push_u16(out: &mut Vec<u8>, v: u16) {
    out.extend(v.to_be_bytes());
}

fn push_i16(out: &mut Vec<u8>, v: i16) {
    out.extend(v.to_be_bytes());
}

fn push_u32(out: &mut Vec<u8>, v: u32) {
    out.extend(v.to_be_bytes());
}

/// The TrueType table checksum: the sum of big-endian `u32`s, zero-padded.
fn checksum(data: &[u8]) -> u32 {
    data.chunks(4).fold(0u32, |sum, chunk| {
        let mut word = [0u8; 4];
        word[..chunk.len()].copy_from_slice(chunk);
        sum.wrapping_add(u32::from_be_bytes(word))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Fonts laid out at `pixels_per_point` 1 from the editor's definitions.
    fn editor_fonts() -> egui::epaint::text::Fonts {
        egui::epaint::text::Fonts::new(1.0, 1024, definitions())
    }

    #[test]
    fn the_editor_uses_only_its_own_fonts() {
        let fonts = definitions();
        let names: Vec<&str> = fonts.font_data.keys().map(String::as_str).collect();
        assert_eq!(names, [ICONS_NAME, PLEX_NAME, INTER_NAME]);
        assert_eq!(
            fonts.families[&FontFamily::Proportional],
            [INTER_NAME, ICONS_NAME]
        );
        assert_eq!(
            fonts.families[&FontFamily::Monospace],
            [INTER_NAME, ICONS_NAME]
        );
        assert_eq!(
            fonts.families[&brand_family()],
            [PLEX_NAME, INTER_NAME, ICONS_NAME]
        );
    }

    /// Every character the editor or the egui widgets it uses can draw as
    /// text has a glyph: the editor's own non-ASCII labels and value units,
    /// and egui's submenu arrow and text-elision ellipsis.
    #[test]
    fn every_drawn_character_has_a_glyph() {
        let fonts = editor_fonts();
        // Not '?': without '◻' in the fonts, egui uses '?' as its replacement
        // glyph, and `has_glyph` reports the replacement glyph as missing
        // (a known false negative in epaint 0.31's `Font::has_glyph`).
        let ascii: String = (' '..='~').filter(|&c| c != '?').collect();
        let drawn = format!("{ascii}…‹›—–×°±µ−→\u{2009}\u{23F4}\u{23F5}\u{23F6}\u{23F7}");
        for family in [FontFamily::Proportional, FontFamily::Monospace] {
            let id = egui::FontId::new(13.0, family.clone());
            let missing: String = drawn
                .chars()
                .filter(|&c| !fonts.has_glyph(&id, c))
                .collect();
            assert!(missing.is_empty(), "{family:?} lacks {missing:?}");
        }
        let brand = egui::FontId::new(13.0, brand_family());
        let brand_text = "CORROSIONINDUSTRIAL RESONATORMODEL: CR-89X";
        assert!(fonts.has_glyphs(&brand, brand_text));
    }

    /// The submenu arrow comes from Corrosion Icons, with a real outline
    /// (not the replacement box), and lines up with Inter: same row height.
    #[test]
    fn the_icon_font_draws_the_submenu_arrow() {
        let fonts = editor_fonts();
        let id = egui::FontId::proportional(13.0);
        let arrow = fonts.layout_no_wrap("⏵".into(), id.clone(), egui::Color32::WHITE);
        let letter = fonts.layout_no_wrap("A".into(), id, egui::Color32::WHITE);
        assert_eq!(arrow.rect.height(), letter.rect.height());
        let glyph = &arrow.rows[0].glyphs[0];
        assert!(glyph.uv_rect.size.x > 0.0 && glyph.uv_rect.size.y > 0.0);
        // The icon font's glyph, not the replacement glyph: the same advance
        // as '⏵' laid out from a family holding only the icon font.
        let mut icons_only = FontDefinitions::empty();
        icons_only.font_data.insert(
            ICONS_NAME.into(),
            Arc::new(FontData::from_owned(icon_font())),
        );
        icons_only
            .families
            .insert(FontFamily::Proportional, vec![ICONS_NAME.into()]);
        let icons_only = egui::epaint::text::Fonts::new(1.0, 1024, icons_only);
        let expected = icons_only.layout_no_wrap(
            "⏵".into(),
            egui::FontId::proportional(13.0),
            egui::Color32::WHITE,
        );
        let expected = expected.rows[0].glyphs[0].advance_width;
        assert_eq!(glyph.advance_width, expected);
    }

    #[test]
    fn the_icon_font_is_well_formed() {
        let font = icon_font();
        // Every table's checksum, and the whole file's (head's
        // checkSumAdjustment makes it 0xB1B0AFBA).
        let num_tables = u16::from_be_bytes([font[4], font[5]]) as usize;
        for i in 0..num_tables {
            let record = &font[12 + 16 * i..28 + 16 * i];
            let tag = &record[0..4];
            let sum = u32::from_be_bytes(record[4..8].try_into().unwrap());
            let offset = u32::from_be_bytes(record[8..12].try_into().unwrap()) as usize;
            let len = u32::from_be_bytes(record[12..16].try_into().unwrap()) as usize;
            let mut data = font[offset..offset + len].to_vec();
            if tag == b"head" {
                data[8..12].copy_from_slice(&[0; 4]);
            }
            assert_eq!(checksum(&data), sum, "{}", String::from_utf8_lossy(tag));
        }
        assert_eq!(checksum(&font), 0xB1B0_AFBA);
        // Deterministic: the same bytes every time.
        assert_eq!(font, icon_font());
    }

    /// Inter at the editor's sizes has the previous font's cap height, and
    /// its rows are no taller than 1 % over the previous font's.
    #[test]
    fn inter_keeps_the_layouts_text_metrics() {
        assert!((INTER_CAP_HEIGHT * UI_EM_SCALE - PREVIOUS_CAP_HEIGHT).abs() < 1e-6);
        let previous_row = (932.0 + 189.0 + 28.0) / 1000.0; // Ubuntu Light's hhea
        let inter_row = (1984.0 + 494.0) / 2048.0 * UI_EM_SCALE;
        assert!(
            inter_row / previous_row < 1.01,
            "{inter_row} vs {previous_row}"
        );
    }
}
