use crate::app::App;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handle key events
pub fn handle_key_event(app: &mut App, key: KeyEvent) {
    match key.code {
        // Quit
        KeyCode::Char('q') | KeyCode::Esc => {
            app.quit();
        }

        // Ctrl+C to quit
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.quit();
        }

        // Navigation
        KeyCode::Down | KeyCode::Char('j') => {
            app.next();
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.previous();
        }
        KeyCode::Char('g') => {
            app.go_to_first();
        }
        KeyCode::Char('G') => {
            app.go_to_last();
        }

        // Page navigation
        KeyCode::PageDown | KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.page_down();
        }
        KeyCode::PageUp | KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.page_up();
        }

        // Actions
        KeyCode::Enter => {
            app.handle_enter();
        }
        KeyCode::Char(' ') => {
            app.toggle_expand();
        }
        KeyCode::Char('o') => {
            app.open_in_browser();
        }
        KeyCode::Char('x') => {
            app.collapse_all();
        }
        KeyCode::Char('r') => {
            app.mark_for_refresh();
        }

        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyEventKind, KeyEventState};

    fn create_key(code: KeyCode) -> KeyEvent {
        KeyEvent {
            code,
            modifiers: KeyModifiers::empty(),
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        }
    }

    #[test]
    fn test_quit_event() {
        let mut app = App::new("http://localhost:3000", 50);
        assert!(!app.should_quit);

        handle_key_event(&mut app, create_key(KeyCode::Char('q')));
        assert!(app.should_quit);
    }
}
