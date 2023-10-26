use std::io::{stdin, stdout};

use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};

#[derive(PartialEq, Debug)]
pub enum PrintCommand {
    AICall,
    UnitTest,
    Issue,
}

impl PrintCommand {
    pub fn print_agent_message(&self, agent_position: &str, agent_statement: &str) {
        let mut stdout = stdout();

        //decide print color
        let statement_color = match self {
            Self::AICall => Color::Cyan,
            Self::UnitTest => Color::Magenta,
            Self::Issue => Color::Red,
        };

        //print agent statement
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        print!("Agent: {}: ", agent_position);

        // make selected color
        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        println!("{}", agent_statement);

        // reset the color
        stdout.execute(ResetColor).unwrap();
    }
}

pub fn get_user_response(question: &str) -> String {
    let mut stdout = stdout();

    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!("");
    println!("{}", question);
    stdout.execute(ResetColor).unwrap();

    let mut uesr_response = String::new();
    stdin()
        .read_line(&mut uesr_response)
        .expect("Failed to read response");

    return uesr_response.trim().to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_prints_agent_message() {
        PrintCommand::AICall
            .print_agent_message("Managing Agent", "Testing testing, processing something");
    }
}
