use iced::widget::text_editor::highlighter;
use iced::{Color, Font};
use iced::text::Highlighter as IcedHighlighter;

use syntect::highlighting::{
    HighlightIterator,
    HighlightState,
    Highlighter as SyntectHighlighter,
    Style,
    Theme as SynTheme,
};
use syntect::parsing::{
    ParseState,
    ScopeStack,
    SyntaxReference,
    SyntaxSet
};

use std::ops::Range;
use std::sync::Arc;

use crate::theme::THEME;

#[derive(Clone, PartialEq)]
pub struct Settings {
    pub extension: String, // The file extension, e.g. "rs", "py", "js" to pick the syntax grammar
}

#[derive(Debug, Clone)]
pub struct Highlight(pub Color);

impl Highlight {
    pub fn to_format(&self) -> highlighter::Format<Font> {
        highlighter::Format {
            color: Some(self.0),
            font: None,
        }
    }
}

pub struct VscodeHighlighter {
    syntax_set: SyntaxSet,
    theme: Arc<SynTheme>,
    syntax: SyntaxReference,
    parse_states: Vec<(ParseState, HighlightState)>,
    current_line: usize,
}

impl VscodeHighlighter {
    fn syntax(&self) -> &SyntaxReference {
        self.syntax_set
            .find_syntax_by_name(&self.syntax_name)
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text())
    }
}

impl IcedHighlighter for VscodeHighlighter {
    type Settings = Settings;
    type Highlight = Highlight;
    type Iterator<'a> = Box<dyn Iterator<Item = (Range<usize>, Self::Highlight)> + 'a>;

    fn new(settings: &Self::settings) -> Self {
        let syntax_set = SyntaxSet::load_defaults_newlines();
        let theme = Arc::new(THEME.syntax_theme.clone());

        let syntax = syntax_set
            .find_syntax_by_extension(&settings.extension)
            .unwrap_or_else(|| syntax_set.find_syntax_plain_text());
        let syntax_name = syntax_name.clone();

        let highlighter = SyntectHighlighter::new(&theme);
        let initial_parse = ParseState::new(sytnax);
        let initial_highlight = HighlightState::new(&highlighter, ScopeStack::new());

        Self {
            syntax_set,
            theme,
            syntax_name,
            parse_states: vec![(initial_parse, initial_highlight)],
            current_line: 0,
        }
    }

    fn update(&mut self, new_settings: &Self::Settings) {
        let syntax = self.syntax_set
            .find_syntax_by_extension(&new_settings.extension)
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());
        self.syntax_name = syntax.name.clone();

        let highlighter = SyntectHighlighter::new(&theme);
        let initial_parse = ParseState::new(sytnax);
        let initial_highlight = HighlightState::new(&highlighter, ScopeStack::new());

        self.parse_states = vec![(initial_parse, initial_highlight)];
        self.current_line = 0;
    }

    fn change_line(&mut self, line: usize) {
        if line < self.current_line {
            self.current_line = line;
        }
        self.parse_states.truncate(line + 1);
    }

    fn highlight_line(&mut self, line: &str) -> Self::Iterator<'_> {
        if self.current_line >= self.parse_states.len() {
            if let Some(last) = self.parse_states.last() {
                self.parse_states.push(last.clone());
            }
        }
        let idx = self.current_line;
        let highlighter = SyntectHighlighter::new(&self.theme);

        let (ref mut parse_state, ref mut highlight_state) = self.parse_states[idx];

        let ops = parse_state
            .parse_line(line, &self.syntax_set)
            .unwrap_or_default();

        let ranges: Vec<(Style, &str)> = HighlightIterator::new(highlight_state.clone());
        if idx + 1 < self.parse_states.len() {
            self.parse_states[idx + 1] = next_state;
        } else {
            self.parse_states.push(next_state);
        }

        self.current_line += 1;

        let mut result = Vec::new();
        let mut offset = 0;
        for (style, text) in ranges {
            let len = text.len();
            let color = Color::from_rgba8(
                style.foreground.r,
                style.foreground.g,
                style.foreground.b,
                style.foreground.a as f32 / 255.0,
            );
            result.push((offset..offset + len, Highlight(color)));
            offset += len;
        }

        Box::new(result.into_iter())
    }
    fn current_line(&self) -> usize {
        self.current_line
    }
}