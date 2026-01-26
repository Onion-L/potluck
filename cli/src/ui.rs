use crate::app::{App, LoadingState};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

const POTLUCK_TITLE: &str = r#"
██████╗  ██████╗ ████████╗██╗     ██╗   ██╗ ██████╗██╗  ██╗
██╔══██╗██╔═══██╗╚══██╔══╝██║     ██║   ██║██╔════╝██║ ██╔╝
██████╔╝██║   ██║   ██║   ██║     ██║   ██║██║     █████╔╝ 
██╔═══╝ ██║   ██║   ██║   ██║     ██║   ██║██║     ██╔═██╗ 
██║     ╚██████╔╝   ██║   ███████╗╚██████╔╝╚██████╗██║  ██╗
╚═╝      ╚═════╝    ╚═╝   ╚══════╝ ╚═════╝  ╚═════╝╚═╝  ╚═╝
"#;

/// Render the UI
pub fn render(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .constraints([
            Constraint::Length(8), // Title
            Constraint::Min(0),    // Content
            Constraint::Length(1), // Footer
        ])
        .split(f.area());

    render_title(f, chunks[0]);
    render_content(f, chunks[1], app);
    render_footer(f, chunks[2]);

    // Render error popup if exists
    if let Some(error) = &app.last_error {
        render_error_popup(f, error);
    }
}

fn render_title(f: &mut Frame, area: Rect) {
    let title = Paragraph::new(POTLUCK_TITLE)
        .style(
            Style::default()
                .fg(Color::Gray)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(ratatui::layout::Alignment::Left);
    f.render_widget(title, area);
}

fn render_content(f: &mut Frame, area: Rect, app: &mut App) {
    match &app.loading_state {
        LoadingState::Loading => {
            render_loading(f, area);
        }
        LoadingState::Error(msg) => {
            render_error(f, area, msg);
        }
        LoadingState::Loaded => {
            render_article_list(f, area, app);
        }
    }
}

fn render_loading(f: &mut Frame, area: Rect) {
    let loading = Paragraph::new("Loading articles...").style(Style::default().fg(Color::Gray));
    f.render_widget(loading, area);
}

fn render_error(f: &mut Frame, area: Rect, msg: &str) {
    let error = Paragraph::new(vec![
        Line::from(Span::styled(
            "Failed to load articles",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(msg, Style::default().fg(Color::DarkGray))),
        Line::from(""),
        Line::from(Span::raw("Press 'r' to retry or 'q' to quit")),
    ]);
    f.render_widget(error, area);
}

fn render_article_list(f: &mut Frame, area: Rect, app: &mut App) {
    // Calculate content width as 85% of terminal width, with minimum of 20
    let content_width = ((area.width as f32) * 0.85).max(20.0) as usize;

    let mut items: Vec<ListItem> = app
        .articles
        .iter()
        .enumerate()
        .map(|(i, article)| {
            let is_expanded = app.expanded.contains(&i);
            create_list_item(article, is_expanded, content_width)
        })
        .collect();

    // Add Website Link item
    let url = &app.client.base_url;
    items.push(ListItem::new(Text::from(vec![
        Line::from(""),
        Line::from(vec![
            Span::raw("   "),
            Span::raw("Want more? Visit "),
            Span::styled(
                format!("{} ↗", url),
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::UNDERLINED),
            ),
        ]),
        Line::from(""),
    ])));

    let list = List::new(items).highlight_style(
        Style::default()
            .bg(Color::Gray)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    f.render_stateful_widget(list, area, &mut app.list_state);
}

fn create_list_item(article: &crate::api::Article, is_expanded: bool, width: usize) -> ListItem<'static> {
    let mut lines = vec![];

    let tag_style = get_tag_style(&article.tag);

    lines.push(Line::from(vec![
        Span::styled(format!("[{}]", article.tag), tag_style),
        Span::raw(" "),
        Span::styled(
            article.title.clone(),
            Style::default().add_modifier(Modifier::BOLD),
        ),
    ]));

    lines.push(Line::from(vec![
        Span::raw("   "),
        Span::styled(
            format!(
                "{} • {}",
                article.source,
                format_time(&article.published_at)
            ),
            Style::default().fg(Color::DarkGray),
        ),
    ]));

    if is_expanded {
        lines.push(Line::from(""));

        if article.summary.is_empty() {
            lines.push(Line::from(vec![
                Span::raw("   "),
                Span::styled(
                    "No summary available",
                    Style::default()
                        .fg(Color::DarkGray)
                        .add_modifier(Modifier::ITALIC),
                ),
            ]));
        } else {
            for line in wrap_text(&article.summary, width.saturating_sub(3)) {
                lines.push(Line::from(vec![
                    Span::raw("   "),
                    Span::styled(line, Style::default().fg(Color::Gray)),
                ]));
            }
        }

        lines.push(Line::from(""));
        for line in wrap_text(&format!("URL: {}", article.url), width.saturating_sub(3)) {
            lines.push(Line::from(vec![
                Span::raw("   "),
                Span::styled(
                    line,
                    Style::default()
                        .fg(Color::DarkGray)
                        .add_modifier(Modifier::UNDERLINED),
                ),
            ]));
        }
        lines.push(Line::from(vec![
            Span::raw("   "),
            Span::styled(
                "Press Enter to open in browser",
                Style::default().fg(Color::White),
            ),
        ]));
    }

    ListItem::new(Text::from(lines))
}

fn get_tag_style(_tag: &str) -> Style {
    Style::default().fg(Color::Gray)
}

fn format_time(iso_time: &str) -> String {
    if iso_time.len() >= 16 {
        let date = &iso_time[0..10];
        let time = &iso_time[11..16];
        format!("{} {}", date, time)
    } else {
        iso_time.to_string()
    }
}

fn wrap_text(text: &str, width: usize) -> Vec<String> {
    if text.is_empty() {
        return Vec::new();
    }

    let mut lines = Vec::new();
    // Ensure minimum width to prevent degenerate cases
    let width = width.max(10);

    // First split by newlines to preserve original line breaks
    for paragraph in text.split('\n') {
        if paragraph.is_empty() {
            lines.push(String::new());
            continue;
        }

        let mut current_line = String::new();
        let mut current_width = 0;

        for ch in paragraph.chars() {
            // Calculate display width: CJK characters take 2 columns
            let char_width = if is_wide_char(ch) { 2 } else { 1 };

            if current_width + char_width > width && !current_line.is_empty() {
                lines.push(current_line);
                current_line = String::new();
                current_width = 0;
            }

            current_line.push(ch);
            current_width += char_width;
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }
    }

    lines
}

/// Check if a character is a wide character (CJK, fullwidth, etc.)
fn is_wide_char(ch: char) -> bool {
    let cp = ch as u32;
    // CJK Unified Ideographs and related ranges
    matches!(cp,
        0x1100..=0x115F |   // Hangul Jamo
        0x2E80..=0x9FFF |   // CJK Radicals, Kangxi, CJK Symbols, Hiragana, Katakana, Bopomofo, Hangul, CJK Ideographs
        0xAC00..=0xD7A3 |   // Hangul Syllables
        0xF900..=0xFAFF |   // CJK Compatibility Ideographs
        0xFE10..=0xFE1F |   // Vertical Forms
        0xFE30..=0xFE6F |   // CJK Compatibility Forms
        0xFF00..=0xFF60 |   // Fullwidth ASCII and Halfwidth CJK punctuation
        0xFFE0..=0xFFE6 |   // Fullwidth symbols
        0x20000..=0x2FFFF | // CJK Extension B-F
        0x30000..=0x3FFFF   // CJK Extension G+
    )
}

fn render_footer(f: &mut Frame, area: Rect) {
    let help_text = vec![
        Span::styled("j/↓", Style::default().fg(Color::White)),
        Span::raw(" Down "),
        Span::styled("k/↑", Style::default().fg(Color::White)),
        Span::raw(" Up "),
        Span::styled("Enter", Style::default().fg(Color::White)),
        Span::raw(" Open "),
        Span::styled("Space", Style::default().fg(Color::White)),
        Span::raw(" Toggle "),
        Span::styled("q", Style::default().fg(Color::White)),
        Span::raw(" Quit"),
    ];

    let footer = Paragraph::new(Line::from(help_text));
    f.render_widget(footer, area);
}

fn render_error_popup(f: &mut Frame, error: &str) {
    let area = f.area();
    let popup_width = 50.min(area.width - 4);
    let popup_height = 5;

    let popup_area = Rect::new(
        (area.width - popup_width) / 2,
        (area.height - popup_height) / 2,
        popup_width,
        popup_height,
    );

    let popup = Paragraph::new(error)
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .title("Error")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Gray)),
        );

    f.render_widget(popup, popup_area);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::App;
    use ratatui::{backend::TestBackend, Terminal};

    #[test]
    fn test_render_does_not_panic() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        let mut app = App::new("http://localhost:3000", 50);

        terminal
            .draw(|f| {
                render(f, &mut app);
            })
            .unwrap();
    }

    #[test]
    fn test_wrap_text_english() {
        let text = "Hello world";
        let lines = wrap_text(text, 20);
        assert_eq!(lines, vec!["Hello world"]);
    }

    #[test]
    fn test_wrap_text_chinese() {
        let text = "你好世界这是一段中文";
        let lines = wrap_text(text, 10);
        // Each Chinese char is 2 columns, so 5 chars per line
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "你好世界这");
        assert_eq!(lines[1], "是一段中文");
    }

    #[test]
    fn test_wrap_text_with_newlines() {
        let text = "Line1\nLine2\nLine3";
        let lines = wrap_text(text, 50);
        assert_eq!(lines, vec!["Line1", "Line2", "Line3"]);
    }

    #[test]
    fn test_wrap_text_empty() {
        let lines = wrap_text("", 50);
        assert!(lines.is_empty());
    }

    #[test]
    fn test_wrap_text_zero_width() {
        // Should use minimum width of 10
        let text = "Hello";
        let lines = wrap_text(text, 0);
        assert_eq!(lines, vec!["Hello"]);
    }

    #[test]
    fn test_is_wide_char() {
        assert!(is_wide_char('中'));
        assert!(is_wide_char('あ'));
        assert!(is_wide_char('한'));
        assert!(!is_wide_char('a'));
        assert!(!is_wide_char('1'));
    }
}
