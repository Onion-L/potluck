use clap::Parser;

/// Potluck TUI - AI-powered tech news reader
#[derive(Parser, Debug, Clone)]
#[command(name = "ptlk")]
#[command(version, about, long_about = None)]
pub struct Config {
    /// API base URL
    #[arg(long, env = "POTLUCK_API_URL")]
    #[cfg_attr(debug_assertions, arg(default_value = "http://localhost:3000"))]
    #[cfg_attr(
        not(debug_assertions),
        arg(default_value = "https://potluck-xl.vercel.app")
    )]
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

        let expected_url = if cfg!(debug_assertions) {
            "http://localhost:3000"
        } else {
            "https://potluck-xl.vercel.app"
        };

        assert_eq!(config.api_url, expected_url);
        assert_eq!(config.limit, 50); // Expect 50
        assert_eq!(config.debug, false);
    }
}
