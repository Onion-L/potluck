use anyhow::Result;
use serde::Deserialize;

/// Article data structure
#[derive(Debug, Deserialize, Clone)]
pub struct Article {
    pub title: String,
    pub url: String,

    #[serde(default)]
    pub summary: String,
    #[serde(default = "default_tag")]
    pub tag: String,
    #[serde(default = "default_source")]
    pub source: String,

    #[serde(rename = "publishedAt")]
    pub published_at: String,
}

fn default_tag() -> String {
    "Tech".to_string()
}

fn default_source() -> String {
    "Unknown".to_string()
}

/// Latest articles response
#[derive(Debug, Deserialize)]
pub struct LatestResponse {
    pub data: Vec<Article>,
}

pub struct ApiClient {
    client: reqwest::Client,
    base_url: String,
}

impl ApiClient {
    pub fn new(base_url: &str) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .user_agent("ptlk/0.1.0")
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
        }
    }

    pub async fn fetch_latest(&self, page: u32, limit: u32) -> Result<LatestResponse> {
        let url = format!(
            "{}/api/latest?page={}&limit={}",
            self.base_url, page, limit
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .json::<LatestResponse>()
            .await?;

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article_deserialization() {
        let json = r#"{
            "title": "Test Article",
            "url": "https://example.com",
            "summary": "Test summary",
            "tag": "AI",
            "source": "TechCrunch",
            "publishedAt": "2024-01-01T00:00:00Z"
        }"#;

        let article: Article = serde_json::from_str(json).unwrap();
        assert_eq!(article.title, "Test Article");
        assert_eq!(article.tag, "AI");
    }

    #[test]
    fn test_article_default_values() {
        let json = r#"{
            "title": "Test",
            "url": "https://example.com",
            "publishedAt": "2024-01-01T00:00:00Z"
        }"#;

        // This will definitely fail because summary, tag, source are missing in JSON and no default is set
        let article: Article = serde_json::from_str(json).unwrap();
        assert_eq!(article.tag, "Tech");
        assert_eq!(article.source, "Unknown");
        assert_eq!(article.summary, "");
    }
}
