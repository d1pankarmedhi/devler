use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::{apis::call_request::call_gpt, models::general::llm::Message};

use super::cmd_line::PrintCommand;

// extend ai function to encourage specific outputs
pub fn extend_ai_function(ai_func: fn(&str) -> &'static str, func_input: &str) -> Message {
    let ai_function_str = ai_func(func_input);

    // extend the string to encourage only printing the output
    let msg = format!("FUNCTION: {}
    INSTRUCTION: You are a function printer. You ONLY print the results of functions. Nothing else. No commentry.
    Here is the input to the function: {}.
    Print out what the function will return.", ai_function_str, func_input);

    Message {
        role: "system".to_string(),
        content: msg,
    }
}

// perform call to LLM GPT
pub async fn ai_task_request(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> String {
    //extend ai function
    let extended_msg = extend_ai_function(function_pass, &msg_context);
    // print current status
    PrintCommand::AICall.print_agent_message(agent_position, agent_operation);

    //get LLM response
    let llm_response_result = call_gpt(vec![extended_msg.clone()]).await;

    //handle success or try again
    let llm_resonse: String = match llm_response_result {
        Ok(llm_res) => llm_res,
        Err(_) => call_gpt(vec![extended_msg].clone())
            .await
            .expect("Failed twice to call OpenAI"),
    };
    llm_resonse
}

// perform call to LLM GPT - decode
pub async fn ai_task_request_decoded<T: DeserializeOwned>(
    msg_context: String,
    agent_position: &str,
    agent_operation: &str,
    function_pass: for<'a> fn(&'a str) -> &'static str,
) -> T {
    let llm_response =
        ai_task_request(msg_context, agent_position, agent_operation, function_pass).await;
    let decoded_response: T = serde_json::from_str(llm_response.as_str())
        .expect("Failed to decode AI response from serde_json");
    decoded_response
}

//check whether request url is valid
pub async fn check_status_code(client: &Client, url: &str) -> Result<u16, reqwest::Error> {
    let response = client.get(url).send().await?;
    Ok(response.status().as_u16())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::ai_functions::managing::convert_user_input_to_goal;
    #[test]
    fn tests_extending_ai_function() {
        let extended_msg = extend_ai_function(convert_user_input_to_goal, "dummy_variable");
        dbg!(&extended_msg);
        assert_eq!(extended_msg.role, "system".to_string());
    }

    #[tokio::test]
    async fn tests_ai_task_request() {
        let ai_func_param = "Build me a webserver for making stock price api request".to_string();
        let res = ai_task_request(
            ai_func_param,
            "Managing Agent",
            "Defining user requirements",
            convert_user_input_to_goal,
        )
        .await;

        assert!(res.len() > 20);
        dbg!(res);
    }
}
