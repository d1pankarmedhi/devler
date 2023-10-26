use ai_functions::ai_function;

#[ai_function]
pub fn print_backend_webserver_code(_project_description_and_template: &str) {
    /// INPUT: Takes in a PROJECT_DESCRIPTION and CODE_TEMPLATE for a website backend build
    /// FUNCTION: Takes an existing set of code marked as CODE_TEMPLATE and updates or re-writes it to work for the porpose in
    /// IMPORTANT: The following libraries are already installed
    ///     reqwest, serde, serde_json, tokio, actix-web, async-trait, actix_cors
    /// Therefore, this function can only work with code from the standard Rust library or the above as per shown in the CODE_TEMPLATE
    /// OUTPUT: Print ONLY the code, nothing else. This function ONLY prints code.
    println!(OUTPUT)
}

#[ai_function]
pub fn print_improved_webserver_code(_project_description_and_template: &str) {
    /// INPUT: takes in a PROJECT_DESCRIPTION and CODE_TEMPLATE for a website backend build
    /// FUNCTION: Performs the following tasks:
    ///     1. Removes any bugs in the code and adds minor additional functionalities
    ///     2. Makes sure everything requested in the spec from backend standpoint was followed. If not, add the feature.
    ///     3. ONLY writes the code. No commmentary.
    /// IMPORTANT: The following libraries are already installed. Does not use any libraries other than what was provided in the description.
    ///
    println!(OUTPUT)
}

#[ai_function]
pub fn print_rest_api_endpoints(_code_input: &str) {
    /// INPUT: Takes in Rust webserver CODE_INPUT on actix-web
    /// FUNCTION: Prints out the JSON schema for url endpoints and their respective types.
    /// LOGIC: Script analyses all code and can categorize into the following object keys:
    ///     "router": This represents the url path of the endpoint
    /// IMPORTANT: only prints out the JSON schema. No commentary or anything else.
    /// MUST READ: All keys are strings. Even bool should be wrapped in a double quotes as "bool"
    ///
    println!(OUTPUT)
}
