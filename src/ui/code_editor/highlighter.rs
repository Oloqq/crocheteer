use std::{hash::Hash, ops::Range};

use bevy::log;
use bevy_egui::egui::{Color32, Stroke, TextFormat, text::LayoutJob};
use egui_code_editor::SemanticAnalyzer;
use strum::IntoEnumIterator;

type StylerFunc = fn(TextFormat) -> TextFormat;

#[derive(strum_macros::EnumIter, Clone, Copy, Debug, Hash)]
#[repr(usize)]
pub enum HighlightLayer {
    LightBackground,
    RedUnderline,
}

impl HighlightLayer {
    fn styler(&self) -> StylerFunc {
        match self {
            HighlightLayer::LightBackground => light_background,
            HighlightLayer::RedUnderline => red_underline,
        }
    }
}

#[derive(Clone, Debug, Hash)]
pub struct Highlighter {
    /// Each type of highlight gets its styler, and list of ranges where it should be applied.
    /// Elements in the back take precedence if two stylers write the same property in the same range.
    highlights: Vec<(HighlightLayer, Vec<Range<usize>>)>,
}

impl Highlighter {
    pub fn new() -> Self {
        Self {
            highlights: HighlightLayer::iter()
                .map(|layer| (layer, vec![]))
                .collect(),
        }
    }

    pub fn set(&mut self, layer: HighlightLayer, mut ranges: Vec<Range<usize>>) {
        self.highlights[layer as usize].1.clear();
        if ranges.is_empty() {
            return;
        }

        ranges.sort_by(|a, b| a.start.cmp(&b.start));
        ranges.dedup();
        let mut non_overlapping = vec![ranges[0].clone()];
        for i in 1..ranges.len() {
            if ranges[i].start <= ranges[i - 1].end {
                non_overlapping.last_mut().unwrap().end = ranges[i - 1].end.max(ranges[i].end);
            } else {
                non_overlapping.push(ranges[i].clone())
            }
        }
        self.highlights[layer as usize]
            .1
            .append(&mut non_overlapping);
    }

    pub fn clear(&mut self, layer: HighlightLayer) {
        self.highlights[layer as usize].1.clear();
    }
}

fn light_background(mut tf: TextFormat) -> TextFormat {
    tf.background = Color32::from_rgba_unmultiplied(255, 255, 255, 60);
    tf
}

fn red_underline(mut tf: TextFormat) -> TextFormat {
    tf.underline = Stroke::new(2.0, Color32::RED);
    tf
}

/// This Analyzer is injected into egui_code_editor.
/// Every time the editor content (or its display settings) change, analyze will be called.
/// egui_code_editor detects change by Hash.
impl SemanticAnalyzer for Highlighter {
    fn analyze(&self, job: LayoutJob) -> LayoutJob {
        if !layout_job_is_continuous(&job) {
            let msg = format!(
                "I expected egui_code_editor to produce a continuous layout job, it did not:\n{}",
                job.text
            );
            log::error!(msg);
            debug_assert!(false);
            return job;
        }

        let mut full_layout: LayoutDef = job
            .sections
            .into_iter()
            .map(|s| (s.byte_range.end, s.format))
            .collect();

        for (layer, ranges) in &self.highlights {
            apply_highlight(&mut full_layout, layer.styler(), ranges.clone());
        }

        let mut new_job = LayoutJob::default();

        let mut last_i = 0;
        for (i, style) in full_layout {
            new_job.append(&job.text[last_i..i], 0.0, style);
            last_i = i;
        }

        new_job
    }
}

/// Complete, continuous formatting of a text.
/// usize in the pair denotes the end of applicable range, so LayoutDef[0].1 defines formatting at characters 0..LayoutDef[0].0, and LayoutDef[1].1 defines formatting for characters in range LayoutDef[0].0..LayoutDef[1].0
type LayoutDef = Vec<(usize, TextFormat)>;

fn apply_highlight(base: &mut LayoutDef, styler: StylerFunc, ranges: Vec<Range<usize>>) {
    if ranges.len() == 0 {
        return;
    }

    let mut result: LayoutDef = Vec::with_capacity(base.len());
    let mut base_i = 0;
    let mut mod_i = 0;
    let mut cursor = 0;

    while base_i < base.len() && mod_i < ranges.len() {
        let base_end = base[base_i].0;
        let mod_start = ranges[mod_i].start;
        let mod_end = {
            if ranges[mod_i].end == mod_start {
                ranges[mod_i].end + 1
            } else {
                ranges[mod_i].end
            }
        };
        let style = base[base_i].1.clone();

        assert!(cursor <= mod_end);

        // applying modification
        if cursor >= mod_start {
            let altered = styler(style);
            if mod_end < base_end {
                result.push((mod_end, altered));
                mod_i += 1;
                cursor = mod_end;
            } else if base_end < mod_end {
                result.push((base_end, altered));
                base_i += 1;
                cursor = base_end;
            } else {
                result.push((base_end, altered));
                base_i += 1;
                mod_i += 1;
                cursor = base_end;
            }
        }
        // applying base
        else {
            if base_end < mod_start {
                result.push((base_end, style));
                base_i += 1;
                cursor = base_end;
            } else {
                result.push((mod_start, style));
                cursor = mod_start;
            }
        }
    }
    while base_i < base.len() {
        result.push(base[base_i].clone());
        base_i += 1;
    }

    *base = result;
}

#[must_use]
fn layout_job_is_continuous(job: &LayoutJob) -> bool {
    let mut last_end = 0;
    for section in &job.sections {
        if section.byte_range.start != last_end {
            return false;
        }
        last_end = section.byte_range.end;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui_code_editor::{ColorTheme, TokenType, format_token};
    use pretty_assertions::assert_eq;

    fn add_italics(mut tf: TextFormat) -> TextFormat {
        tf.italics = true;
        tf
    }

    #[test]
    fn test_merge_styles_no_modifications() {
        let text: String = "0123456789".into();
        assert_eq!(text.len(), 10);
        let theme = ColorTheme::GRUVBOX_DARK;
        let literal = format_token(&theme, 14.0, TokenType::Literal);
        let keyword = format_token(&theme, 14.0, TokenType::Keyword);
        let function = format_token(&theme, 14.0, TokenType::Function);

        let syntax_formats: LayoutDef = vec![
            (2, literal.clone()),
            (9, keyword.clone()),
            (10, function.clone()),
        ]
        .into();

        let mut clone = syntax_formats.clone();
        apply_highlight(&mut clone, add_italics, vec![]);

        assert_eq!(syntax_formats, clone);
    }

    #[test]
    fn test_merge_styles_single_modification_beginning() {
        let text: String = "0123456789".into();
        assert_eq!(text.len(), 10);
        let theme = ColorTheme::GRUVBOX_DARK;
        let literal = format_token(&theme, 14.0, TokenType::Literal);
        let mut ital = literal.clone();
        ital.italics = true;

        let mut full_format: LayoutDef = vec![(10, literal.clone())].into();

        apply_highlight(&mut full_format, add_italics, vec![0..2]);

        assert_eq!(full_format.len(), 2);
        assert_eq!(full_format[0], (2, ital));
        assert_eq!(full_format[1], (10, literal.clone()));
    }

    #[test]
    fn test_merge_styles_single_modification_end() {
        let text: String = "0123456789".into();
        assert_eq!(text.len(), 10);
        let theme = ColorTheme::GRUVBOX_DARK;
        let literal = format_token(&theme, 14.0, TokenType::Literal);
        let mut ital = literal.clone();
        ital.italics = true;

        let mut full_format: LayoutDef = vec![(10, literal.clone())].into();

        apply_highlight(&mut full_format, add_italics, vec![8..10]);

        assert_eq!(full_format[0], (8, literal.clone()));
        assert_eq!(full_format[1], (10, ital));
        assert_eq!(full_format.get(2), None);
    }

    #[test]
    fn test_merge_styles_single_modification_within() {
        let text: String = "0123456789".into();
        assert_eq!(text.len(), 10);
        let theme = ColorTheme::GRUVBOX_DARK;
        let literal = format_token(&theme, 14.0, TokenType::Literal);
        let mut ital = literal.clone();
        ital.italics = true;

        let mut full_format: LayoutDef = vec![(10, literal.clone())].into();

        apply_highlight(&mut full_format, add_italics, vec![3..8]);

        assert_eq!(full_format.len(), 3);
        assert_eq!(full_format[0], (3, literal.clone()));
        assert_eq!(full_format[1], (8, ital));
        assert_eq!(full_format[2], (10, literal.clone()));
    }

    #[test]
    fn test_merge_styles_base_swaps_mid_modification() {
        let text: String = "0123456789".into();
        assert_eq!(text.len(), 10);
        let theme = ColorTheme::GRUVBOX_DARK;
        let literal = format_token(&theme, 14.0, TokenType::Literal);
        let keyword = format_token(&theme, 14.0, TokenType::Keyword);
        let mut ital_lit = literal.clone();
        ital_lit.italics = true;
        let mut ital_kw = keyword.clone();
        ital_kw.italics = true;

        let mut full_format: LayoutDef = vec![(5, literal.clone()), (10, keyword.clone())].into();

        apply_highlight(&mut full_format, add_italics, vec![3..8]);

        assert_eq!(full_format.len(), 4);
        assert_eq!(full_format[0], (3, literal.clone()));
        assert_eq!(full_format[1], (5, ital_lit.clone()));
        assert_eq!(full_format[2], (8, ital_kw));
        assert_eq!(full_format[3], (10, keyword.clone()));
    }

    #[test]
    fn test_merge_styles_zero_width_is_displayed_as_one_width() {
        let text: String = "0123456789".into();
        assert_eq!(text.len(), 10);
        let theme = ColorTheme::GRUVBOX_DARK;
        let literal = format_token(&theme, 14.0, TokenType::Literal);
        let keyword = format_token(&theme, 14.0, TokenType::Keyword);
        let mut ital_lit = literal.clone();
        ital_lit.italics = true;
        let mut ital_kw = keyword.clone();
        ital_kw.italics = true;

        let mut full_format: LayoutDef = vec![(10, literal.clone())].into();

        apply_highlight(&mut full_format, add_italics, vec![2..2]);

        assert_eq!(full_format.len(), 3);
        assert_eq!(full_format[0], (2, literal.clone()));
        assert_eq!(full_format[1], (3, ital_lit.clone()));
        assert_eq!(full_format[2], (10, literal.clone()));
    }

    #[test]
    fn test_highlighter_prevents_repeated_ranges() {
        let mut h = Highlighter::new();
        h.set(HighlightLayer::LightBackground, vec![2..8, 2..8]);
        assert_eq!(
            h.highlights[HighlightLayer::LightBackground as usize]
                .1
                .len(),
            1
        );
    }

    #[test]
    fn test_highlighter_prevents_out_of_order() {
        let mut h = Highlighter::new();
        h.set(HighlightLayer::LightBackground, vec![5..8, 2..4]);
        let ranges = &h.highlights[HighlightLayer::LightBackground as usize].1;
        assert_eq!(ranges.len(), 2);
        assert_eq!(ranges[0], 2..4);
        assert_eq!(ranges[1], 5..8);
    }

    #[test]
    fn test_highlighter_prevents_overlaps() {
        let mut h = Highlighter::new();
        h.set(HighlightLayer::LightBackground, vec![2..5, 4..8]);
        let ranges = &h.highlights[HighlightLayer::LightBackground as usize].1;
        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0], 2..8);

        h.set(HighlightLayer::LightBackground, vec![2..5, 5..8]);
        let ranges = &h.highlights[HighlightLayer::LightBackground as usize].1;
        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0], 2..8);

        h.set(HighlightLayer::LightBackground, vec![2..8, 3..5]);
        let ranges = &h.highlights[HighlightLayer::LightBackground as usize].1;
        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0], 2..8);

        h.set(HighlightLayer::LightBackground, vec![2..5, 5..8, 8..10]);
        let ranges = &h.highlights[HighlightLayer::LightBackground as usize].1;
        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0], 2..10);

        h.set(HighlightLayer::LightBackground, vec![2..5, 4..8, 7..10]);
        let ranges = &h.highlights[HighlightLayer::LightBackground as usize].1;
        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0], 2..10);
    }
}
