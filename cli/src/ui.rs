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
    let mut items: Vec<ListItem> = app
        .articles
        .iter()
        .enumerate()
        .map(|(i, article)| {
            let is_expanded = app.expanded.contains(&i);
            create_list_item(article, is_expanded)
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

fn create_list_item(article: &crate::api::Article, is_expanded: bool) -> ListItem<'static> {
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
            for line in wrap_text(&article.summary, 70) {
                lines.push(Line::from(vec![
                    Span::raw("   "),
                    Span::styled(line, Style::default().fg(Color::Gray)),
                ]));
            }
        }

        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::raw("   "),
            Span::styled(
                format!("URL: {}", article.url),
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::UNDERLINED),
            ),
        ]));
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
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        if current_line.len() + word.len() + 1 > width && !current_line.is_empty() {
            lines.push(current_line);
            current_line = String::new();
        }
        if !current_line.is_empty() {
            current_line.push(' ');
        }
        current_line.push_str(word);
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
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
}
