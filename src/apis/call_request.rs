use crate::models::general::llm::{ApiResponse, ChatCompletion, Message};
use dotenv::dotenv;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use std::env;

//call llm model (GPT 35)
pub async fn call_gpt(messages: Vec<Message>) -> Result<String, Box<dyn std::error::Error + Send>> {
    // send is to transfer the Error across threads

    dotenv().ok();

    // extract api key
    let api_key: String =
        env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found in the environment variables");
    let api_org: String =
        env::var("OPENAI_ORG").expect("OPENAI_ORG not found in the environment variables");

    // confirm endpoint
    let url: &str = "https://api.openai.com/v1/chat/completions";

    // create headers
    let mut headers: HeaderMap = HeaderMap::new();

    //create api key header
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key))
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    //create api org
    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(&api_org.as_str())
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    // create client
    let client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    //create chatcompletion
    let chat_completion = ChatCompletion {
        model: "gpt-3.5-turbo".to_string(),
        messages: messages,
        temperature: 0.1,
    };

    let res_raw = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .unwrap();

    // Extract api response
    let response: ApiResponse = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
        .json()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    // send response
    Ok(response.choices[0].message.content.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_call_to_openai() {
        let message = Message {
            role: "user".to_string(),
            content: "Hi there, this is a test! Give a response".to_string(),
        };

        let messages = vec![message];
        let res = call_gpt(messages).await;
        match res {
            Ok(res_str) => {
                dbg!(res_str);
                assert!(true);
            }
            Err(_) => {
                assert!(false);
            }
        }
    }
}
