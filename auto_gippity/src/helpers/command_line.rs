use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand
};
use std::io::{stdin, stdout};

pub fn get_user_response(question: &str) -> String {
    let mut stdout: std::io::Stdout = stdout();

    // Print the questyion in a specific color
    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!("");
    println!("{}", question);

    // Reset Color
    stdout.execute(ResetColor).unwrap();

    // Read user input
    let mut user_reponse: String = String::new();
    stdin() 
        .read_line(&mut user_reponse)
        .expect("failed to read response");
}
