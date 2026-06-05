use once_cell::sync::Lazy;
use pulldown_cmark::{CodeBlockKind, CowStr, Event, Options, Parser, Tag, TagEnd};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use syntect::easy::HighlightLines;
use syntect::highlighting::{FontStyle, Style as SyntectStyle, ThemeSet};
use syntect::parsing::{SyntaxReference, SyntaxSet};
use vtcode_commons::diff_paths::{
    is_diff_addition_line, is_diff_deletion_line, is_diff_header_line, looks_like_diff_content,
};

static SYNTAX_SET: Lazy<SyntaxSet> = Lazy::new(SyntaxSet::load_defaults_newlines);
static THEME_SET: Lazy<ThemeSet> = Lazy::new(ThemeSet::load_defaults);

#[derive(Debug, Clone, Copy)]
pub struct MarkdownStyleSheet {
    pub text: Style,
    pub strong: Style,
    pub emphasis: Style,
    pub inline_code: Style,
    pub code: Style,
    pub heading: Style,
    pub quote: Style,
    pub link: Style,
    pub bullet: Style,
    pub rule: Style,
    /// Style for unified-diff addition lines (`+...`).
    pub diff_added: Style,
    /// Style for unified-diff deletion lines (`-...`).
    pub diff_removed: Style,
    /// Style for unified-diff metadata/header lines (`diff --git`, `@@`, `+++`, `---`, …).
    pub diff_header: Style,
}

impl Default for MarkdownStyleSheet {
    fn default() -> Self {
        Self {
            text: Style::default(),
            strong: Style::default().add_modifier(Modifier::BOLD),
            emphasis: Style::default().add_modifier(Modifier::ITALIC),
            inline_code: Style::default().fg(Color::Cyan),
            code: Style::default().fg(Color::Gray),
            heading: Style::default().add_modifier(Modifier::BOLD),
            quote: Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::ITALIC),
            link: Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::UNDERLINED),
            bullet: Style::default().fg(Color::Yellow),
            rule: Style::default().fg(Color::DarkGray),
            diff_added: Style::default().fg(Color::Green),
            diff_removed: Style::default().fg(Color::Red),
            diff_header: Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        }
    }
}

pub fn render_markdown(source: &str, styles: MarkdownStyleSheet) -> Vec<Line<'static>> {
    let options =
        Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TABLES | Options::ENABLE_TASKLISTS;
    let mut parser = Parser::new_ext(source, options).peekable();
    let mut lines: Vec<Line<'static>> = Vec::new();
    let mut current: Vec<Span<'static>> = Vec::new();
    let mut style_stack = vec![styles.text];
    let mut list_depth = 0usize;
    let mut ordered_stack: Vec<Option<u64>> = Vec::new();
    let mut pending_link_target: Option<String> = None;

    while let Some(event) = parser.next() {
        match event {
            Event::Start(tag) => match tag {
                Tag::Paragraph => {}
                Tag::Heading { .. } => style_stack.push(styles.heading),
                Tag::Strong => style_stack.push(styles.strong),
                Tag::Emphasis => style_stack.push(styles.emphasis),
                Tag::BlockQuote(_) => style_stack.push(styles.quote),
                Tag::List(start) => {
                    list_depth += 1;
                    ordered_stack.push(start);
                }
                Tag::Item => {
                    if !current.is_empty() {
                        lines.push(Line::from(std::mem::take(&mut current)));
                    }
                    let indent = "  ".repeat(list_depth.saturating_sub(1));
                    let marker = match ordered_stack.last().copied().flatten() {
                        Some(start) => format!("{}{}. ", indent, start),
                        None => format!("{}• ", indent),
                    };
                    current.push(Span::styled(marker, styles.bullet));
                    if let Some(Some(start)) = ordered_stack.last_mut() {
                        *start += 1;
                    }
                }
                Tag::Link { dest_url, .. } => {
                    pending_link_target = Some(dest_url.to_string());
                    style_stack.push(styles.link);
                }
                Tag::CodeBlock(kind) => {
                    if !current.is_empty() {
                        lines.push(Line::from(std::mem::take(&mut current)));
                    }
                    let language = match kind {
                        CodeBlockKind::Fenced(lang) => lang.to_string(),
                        CodeBlockKind::Indented => String::new(),
                    };
                    let mut code = String::new();
                    for next in parser.by_ref() {
                        match next {
                            Event::End(TagEnd::CodeBlock) => break,
                            Event::Text(text) | Event::Code(text) => code.push_str(&text),
                            Event::SoftBreak | Event::HardBreak => code.push('\n'),
                            _ => {}
                        }
                    }
                    render_code_block(&mut lines, &language, &code, styles);
                }
                _ => {}
            },
            Event::End(end) => match end {
                TagEnd::Paragraph => {
                    if !current.is_empty() {
                        lines.push(Line::from(std::mem::take(&mut current)));
                    }
                    lines.push(Line::default());
                }
                TagEnd::Heading(_) | TagEnd::Strong | TagEnd::Emphasis | TagEnd::BlockQuote(_) => {
                    style_stack.pop();
                }
                TagEnd::List(_) => {
                    list_depth = list_depth.saturating_sub(1);
                    ordered_stack.pop();
                    if !current.is_empty() {
                        lines.push(Line::from(std::mem::take(&mut current)));
                    }
                }
                TagEnd::Item => {
                    if !current.is_empty() {
                        lines.push(Line::from(std::mem::take(&mut current)));
                    }
                }
                TagEnd::Link => {
                    style_stack.pop();
                    if let Some(link_target) = pending_link_target.take() {
                        current.push(Span::styled(format!(" ({link_target})"), styles.link));
                    }
                }
                _ => {}
            },
            Event::Text(text) => append_text(
                &mut current,
                &text,
                *style_stack.last().unwrap_or(&styles.text),
            ),
            Event::Code(text) => current.push(Span::styled(text.to_string(), styles.inline_code)),
            Event::SoftBreak => current.push(Span::raw(" ")),
            Event::HardBreak => {
                lines.push(Line::from(std::mem::take(&mut current)));
            }
            Event::Rule => {
                if !current.is_empty() {
                    lines.push(Line::from(std::mem::take(&mut current)));
                }
                lines.push(Line::from(Span::styled("─".repeat(32), styles.rule)));
            }
            Event::TaskListMarker(checked) => {
                let marker = if checked { "[x] " } else { "[ ] " };
                current.push(Span::styled(marker, styles.bullet));
            }
            Event::Html(html) | Event::InlineHtml(html) => {
                append_text(
                    &mut current,
                    &html,
                    *style_stack.last().unwrap_or(&styles.text),
                );
            }
            Event::InlineMath(text) => {
                current.push(Span::styled(format!("${text}$"), styles.inline_code))
            }
            Event::DisplayMath(text) => {
                if !current.is_empty() {
                    lines.push(Line::from(std::mem::take(&mut current)));
                }
                lines.push(Line::from(Span::styled(
                    format!("$$ {text} $$"),
                    styles.inline_code,
                )));
            }
            Event::FootnoteReference(reference) => {
                current.push(Span::styled(format!("[^{reference}]"), styles.link));
            }
        }
    }

    if !current.is_empty() {
        lines.push(Line::from(current));
    }

    while lines.last().is_some_and(|line| line.width() == 0) {
        lines.pop();
    }

    if lines.is_empty() {
        vec![Line::default()]
    } else {
        lines
    }
}

fn append_text(current: &mut Vec<Span<'static>>, text: &CowStr<'_>, style: Style) {
    for (index, line) in text.lines().enumerate() {
        if index > 0 {
            current.push(Span::raw("\n"));
        }
        if !line.is_empty() {
            current.push(Span::styled(line.to_string(), style));
        }
    }
}

fn render_code_block(
    lines: &mut Vec<Line<'static>>,
    language: &str,
    code: &str,
    styles: MarkdownStyleSheet,
) {
    let fence = if language.is_empty() {
        "```".to_string()
    } else {
        format!("```{language}")
    };
    lines.push(Line::from(Span::styled(fence, styles.quote)));
    if is_diff_language(language) || (language.is_empty() && looks_like_diff_content(code)) {
        render_diff_lines(lines, code, styles);
    } else {
        for line in highlight_code(code, language, styles) {
            lines.push(line);
        }
    }
    lines.push(Line::from(Span::styled("```", styles.quote)));
}

fn is_diff_language(language: &str) -> bool {
    matches!(
        language.trim().to_ascii_lowercase().as_str(),
        "diff" | "patch" | "udiff" | "git"
    )
}

/// Render a unified/git diff with addition/deletion/header coloring. Each line is
/// classified via the shared `vtcode_commons::diff_paths` heuristics so the same
/// rules apply everywhere diffs are shown.
fn render_diff_lines(lines: &mut Vec<Line<'static>>, code: &str, styles: MarkdownStyleSheet) {
    for raw in code.lines() {
        let trimmed = raw.trim_start();
        let style = if trimmed.is_empty() {
            styles.code
        } else if is_diff_header_line(trimmed) {
            styles.diff_header
        } else if is_diff_addition_line(trimmed) {
            styles.diff_added
        } else if is_diff_deletion_line(trimmed) {
            styles.diff_removed
        } else {
            styles.code
        };
        lines.push(Line::from(Span::styled(raw.to_string(), style)));
    }
}

fn highlight_code(code: &str, language: &str, styles: MarkdownStyleSheet) -> Vec<Line<'static>> {
    let theme = THEME_SET
        .themes
        .get("base16-ocean.dark")
        .or_else(|| THEME_SET.themes.values().next())
        .cloned()
        .unwrap_or_default();
    let syntax = find_syntax(language);
    let mut highlighter = HighlightLines::new(syntax, &theme);
    let mut lines = Vec::new();
    for raw_line in code.lines() {
        let segments = highlighter
            .highlight_line(raw_line, &SYNTAX_SET)
            .unwrap_or_default();
        if segments.is_empty() {
            lines.push(Line::from(Span::styled(raw_line.to_string(), styles.code)));
            continue;
        }
        let spans = segments
            .into_iter()
            .map(|(style, text)| Span::styled(text.to_string(), syntect_style_to_ratatui(style)))
            .collect::<Vec<_>>();
        lines.push(Line::from(spans));
    }
    if code.ends_with('\n') {
        lines.push(Line::default());
    }
    lines
}

fn find_syntax(language: &str) -> &'static SyntaxReference {
    if !language.is_empty() {
        if let Some(syntax) = SYNTAX_SET.find_syntax_by_token(language) {
            return syntax;
        }
        if let Some(syntax) = SYNTAX_SET.find_syntax_by_extension(language) {
            return syntax;
        }
    }
    SYNTAX_SET.find_syntax_plain_text()
}

fn syntect_style_to_ratatui(style: SyntectStyle) -> Style {
    let mut result = Style::default().fg(Color::Rgb(
        style.foreground.r,
        style.foreground.g,
        style.foreground.b,
    ));
    if style.background.a != 0 {
        result = result.bg(Color::Rgb(
            style.background.r,
            style.background.g,
            style.background.b,
        ));
    }
    if style.font_style.contains(FontStyle::BOLD) {
        result = result.add_modifier(Modifier::BOLD);
    }
    if style.font_style.contains(FontStyle::ITALIC) {
        result = result.add_modifier(Modifier::ITALIC);
    }
    if style.font_style.contains(FontStyle::UNDERLINE) {
        result = result.add_modifier(Modifier::UNDERLINED);
    }
    result
}
