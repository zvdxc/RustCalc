extern crate meval;
use std::process::Command;
use std::str;
use std::thread;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let use_terminal = args.len() > 1 && args[1] == "cmd";

    loop {
        let expression = if use_terminal {
            println!("Enter an expression:");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            input.trim().to_string()
        } else {
            let output = Command::new("zenity")
                .args(&["--entry", "--title=Calculator", "--text=Enter an expression:"])
                .output()
                .expect("Failed to execute zenity");
            str::from_utf8(&output.stdout).expect("Invalid UTF-8").trim().to_string()
        };

        if !expression.is_empty() {
            match meval::eval_str(&expression) {
                Ok(result) => {
                    let result_str = result.to_string();
                    if result_str == "42" {
                        thread::spawn(move || {
                            Command::new("mpv")
                                .args(&["/home/jannis/rick.mp4"])
                                .output()
                                .expect("Failed to execute mpv");
                        });
                    } else {
                        let result_message = format!("Result: {}", result);
                        if use_terminal {
                            println!("{}", result_message);
                        } else {
                            thread::spawn(move || {
                                Command::new("zenity")
                                    .args(&["--info", "--text", &result_message])
                                    .output()
                                    .expect("Failed to execute zenity");
                            });
                        }
                    }
                }
                Err(_) => {
                    let error_message = "Invalid expression";
                    if use_terminal {
                        eprintln!("{}", error_message);
                    } else {
                        thread::spawn(move || {
                            Command::new("zenity")
                                .args(&["--error", "--text=Invalid expression"])
                                .output()
                                .expect("Failed to execute zenity");
                        });
                    }
                }
            }
        } else {
            break;
        }
    }
}