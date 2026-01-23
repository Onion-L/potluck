use crate::api::{ApiClient, Article};
use anyhow::Result;
use ratatui::widgets::ListState;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub enum LoadingState {
    Loading,
    Loaded,
    Error(String),
}

pub struct App {
    pub articles: Vec<Article>,
    pub list_state: ListState,
    pub expanded: HashSet<usize>,
    pub loading_state: LoadingState,
    client: ApiClient,
    limit: u32,
    pub should_quit: bool,
    pub last_error: Option<String>,
    pub needs_refresh: bool,
}

impl App {
    pub fn new(api_url: &str, limit: u32) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            articles: Vec::new(),
            list_state,
            expanded: HashSet::new(),
            loading_state: LoadingState::Loading,
            client: ApiClient::new(api_url),
            limit,
            should_quit: false,
            last_error: None,
            needs_refresh: false,
        }
    }

    pub async fn load_initial(&mut self) -> Result<()> {
        self.loading_state = LoadingState::Loading;
        match self.client.fetch_latest(1, self.limit).await {
            Ok(response) => {
                self.articles = response.data;
                self.loading_state = LoadingState::Loaded;
                if !self.articles.is_empty() {
                    self.list_state.select(Some(0));
                }
            }
            Err(e) => {
                self.loading_state = LoadingState::Error(e.to_string());
            }
        }
        Ok(())
    }

    pub async fn refresh(&mut self) -> Result<()> {
        self.expanded.clear();
        self.load_initial().await
    }

    pub fn next(&mut self) {
        if self.articles.is_empty() {
            return;
        }

        let total_items = self.articles.len() + 1; // +1 for Website Link

        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= total_items - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        if self.articles.is_empty() {
            return;
        }

        let total_items = self.articles.len() + 1; // +1 for Website Link

        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    total_items - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn go_to_first(&mut self) {
        if !self.articles.is_empty() {
            self.list_state.select(Some(0));
        }
    }

    pub fn go_to_last(&mut self) {
        if !self.articles.is_empty() {
            self.list_state.select(Some(self.articles.len())); // Last item (Website Link)
        }
    }

    pub fn page_down(&mut self) {
        if self.articles.is_empty() {
            return;
        }

        let total_items = self.articles.len() + 1;
        let current = self.list_state.selected().unwrap_or(0);
        let new_index = (current + 10).min(total_items - 1);
        self.list_state.select(Some(new_index));
    }

    pub fn page_up(&mut self) {
        let current = self.list_state.selected().unwrap_or(0);
        let new_index = current.saturating_sub(10);
        self.list_state.select(Some(new_index));
    }

    pub fn toggle_expand(&mut self) {
        if let Some(i) = self.list_state.selected() {
            if i == self.articles.len() {
                return; // Cannot expand website link
            }
            if self.expanded.contains(&i) {
                self.expanded.remove(&i);
            } else {
                self.expanded.insert(i);
            }
        }
    }

    pub fn is_current_expanded(&self) -> bool {
        self.list_state
            .selected()
            .map(|i| self.expanded.contains(&i))
            .unwrap_or(false)
    }

    #[allow(dead_code)]
    pub fn selected_article(&self) -> Option<&Article> {
        self.list_state
            .selected()
            .filter(|&i| i < self.articles.len()) // Ensure not the last item
            .and_then(|i| self.articles.get(i))
    }

    pub fn open_in_browser(&mut self) {
        if let Some(i) = self.list_state.selected() {
            if i == self.articles.len() {
                // Website Link selected
                self.open_website();
            } else if let Some(article) = self.articles.get(i) {
                if let Err(e) = open::that(&article.url) {
                    self.last_error = Some(format!("Failed to open browser: {}", e));
                }
            }
        }
    }

    pub fn open_website(&mut self) {
        let url = &self.client.base_url;
        if let Err(e) = open::that(url) {
            self.last_error = Some(format!("Failed to open website: {}", e));
        }
    }

    pub fn handle_enter(&mut self) {
        if let Some(i) = self.list_state.selected() {
            if i == self.articles.len() {
                self.open_website();
            } else if self.is_current_expanded() {
                self.open_in_browser();
            } else {
                self.toggle_expand();
            }
        }
    }

    pub fn collapse_all(&mut self) {
        self.expanded.clear();
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn mark_for_refresh(&mut self) {
        self.needs_refresh = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_app() -> App {
        let mut app = App::new("http://localhost:3000", 50);
        app.articles = vec![
            Article {
                title: "Article 1".to_string(),
                url: "https://example.com/1".to_string(),
                summary: "Summary 1".to_string(),
                tag: "Tech".to_string(),
                source: "Source 1".to_string(),
                published_at: "2024-01-01T00:00:00Z".to_string(),
            },
            Article {
                title: "Article 2".to_string(),
                url: "https://example.com/2".to_string(),
                summary: "Summary 2".to_string(),
                tag: "AI".to_string(),
                source: "Source 2".to_string(),
                published_at: "2024-01-02T00:00:00Z".to_string(),
            },
        ];
        app.loading_state = LoadingState::Loaded;
        app
    }

    #[test]
    fn test_navigation() {
        let mut app = create_test_app();

        // Articles length is 2. Total items: 3 (0, 1, 2)
        // 0: Article 1
        // 1: Article 2
        // 2: Website Link

        assert_eq!(app.list_state.selected(), Some(0));

        app.next();
        assert_eq!(app.list_state.selected(), Some(1));

        app.next();
        assert_eq!(app.list_state.selected(), Some(2)); // Website Link

        app.next();
        assert_eq!(app.list_state.selected(), Some(0)); // Loop back to start

        app.previous();
        assert_eq!(app.list_state.selected(), Some(2)); // Loop back to end
    }

    #[test]
    fn test_expand_toggle() {
        let mut app = create_test_app();

        assert!(!app.is_current_expanded());

        app.toggle_expand();
        assert!(app.is_current_expanded());

        app.toggle_expand();
        assert!(!app.is_current_expanded());
    }
}
