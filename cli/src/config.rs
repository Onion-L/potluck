use clap::Parser;

/// Potluck TUI - AI-powered tech news reader
#[derive(Parser, Debug, Clone)]
#[command(name = "ptlk")]
#[command(version, about, long_about = None)]
pub struct Config {
    /// API base URL
    #[arg(long, env = "POTLUCK_API_URL", default_value = "http://localhost:3000")]
    pub api_url: String,

    /// Number of articles to fetch per page
    #[arg(short, long, default_value = "50")]
    pub limit: u32,

    /// Enable debug mode
    #[arg(short, long, default_value = "false")]
    pub debug: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        // Simulate parsing with no arguments
        let config = Config::parse_from(&["ptlk"]);
        assert_eq!(config.api_url, "http://localhost:3000"); // Expect 3000
        assert_eq!(config.limit, 50); // Expect 50
        assert_eq!(config.debug, false);
    }
}
