use ai_functions::ai_function;

#[ai_function]
pub fn print_project_scope(_project_description: &str) {
    /// Input: Takes in a user request to build a website project description
    /// Function: Converts user request into JSON response of information items required for a wesite build.
    /// Important: At least one of the bool results must be true
    /// Output: Prints an object response in the following format:
    ///     {
    ///         "is_crud_required": bool, // true if site needs CRUD functionally
    ///         "is_user_login_and_logout": bool // true if site needs users to be able to login and logout
    ///         "is_external_urls_required": bool //true if site needs to fetch data from third part providers
    ///     }
    ///
    /// Example 1:
    ///   user_request = "I need a full stack website that accepts users and gets stock price data"
    ///   prints:
    ///     {
    ///         "is_crud_required": true
    ///         "is_user_login_and_logout": true
    ///         "is_external_urls_required": bool true
    ///     }
    ///
    println!(OUTPUT)
}

#[ai_function]
pub fn print_project_site_urls(_project_description: &str) {
    /// Input: Takes in a prject description of a website build
    /// Function: Outputs a list of external public API endpoints that should be used in building the website
    /// Important: Only select url endpoint(s) which do not require any API keys at all
    /// Output: Prints a list response of external urls in the following format:
    ///     ["url1", "url2", "url3", ...]
    /// Example:
    ///     website_team_spec = "website_purpose: Some("\"Provides Crypto Price Data from Binance and Kraken\"",)"
    ///     prints:
    ///         ["https://api.binance.com/api/v3/exchangeInfo", "https://api.binance.com/api/v3/klines?symbol=BTCUSDT&interval=1d"]
    println!(OUTPUT)
}
