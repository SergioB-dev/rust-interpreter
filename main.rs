use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;
use std::str::Chars;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            // Uncomment this block to pass the first stage
            if !file_contents.is_empty() {
                tokenize(&file_contents);
            } else {
                 println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
             }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}

/// A char combo is a sequence of two chars whose individual values
/// are invalid, e.g. != , the ! should not be considered without the
/// = and vice versa
// fn lookahead_possible_char_combo(lookup_char: char, chars: &Chars, counter: usize) -> bool {
//     match chars.nth(counter + 1) {
//         Some(n_char) => {
//
//         }
//         None => { }
//
//     }
// }


fn tokenize(file_content: &str) {
    let mut line_counter = 1;
    let mut char_counter = 0;
    let mut parse_error = 0; // 0 represents no error
    let mut should_skip_equal_sign = false;
    let mut should_skip_bang = false;
    let mut should_skip_slash = false;
    let mut should_skip_rest_of_line = false;

    for char in file_content.chars() {
        if should_skip_rest_of_line { break }
        match char {
            '(' => { println!("LEFT_PAREN ( null") }
            ')' => { println!("RIGHT_PAREN ) null") }
            '{' => { println!("LEFT_BRACE {{ null") }
            '}' => { println!("RIGHT_BRACE }} null") }
            '*' => { println!("STAR * null") }
            '.' => { println!("DOT . null") }
            ',' => { println!("COMMA , null") }
            '+' => { println!("PLUS + null") }
            '-' => { println!("MINUS - null") }
            ';' => { println!("SEMICOLON ; null") }
            '/' => {
                    match file_content.chars().nth(char_counter + 1) {
                        Some(n_ahead_one) => {
                            if n_ahead_one == '/' {
                                should_skip_rest_of_line = true;
                            } // Then this is a comment, don't tokenize
                        }
                        None => {
                            println!("SLASH / null") // Then this is one single slash, tokenize
                        }
                    }
            }
            '<' => {
                    match file_content.chars().nth(char_counter + 1) {
                        Some(n_ahead_one) => {
                            if n_ahead_one == '=' {
                                println!("LESS_EQUAL <= null");
                                should_skip_equal_sign = true;
                            } else {
                                println!("LESS < null")
                            }
                        }
                        None => {
                            println!("LESS < null")
                        }
                    }
            }
            '>' => {
                    match file_content.chars().nth(char_counter + 1) {
                        Some(n_ahead_one) => {
                            if n_ahead_one == '=' {
                                println!("GREATER_EQUAL >= null");
                                should_skip_equal_sign = true;
                            } else {
                                println!("GREATER > null")
                            }
                        }
                        None => {
                            println!("GREATER > null")
                        }
                    }
            }
            '!' => {
                match file_content.chars().nth(char_counter) {
                    Some(n_char) => {
                        if !should_skip_bang {
                            match file_content.chars().nth(char_counter + 1) {
                                Some(n_ahead_one) => {
                                    if n_ahead_one == '=' {
                                        println!("BANG_EQUAL != null");
                                        should_skip_bang = true;
                                        should_skip_equal_sign = true;
                                    } else {
                                        println!("BANG ! null");
                                    }
                                }
                                None => {}
                            }
                        } else {
                            should_skip_bang = false;
                            should_skip_equal_sign = false;
                        }
                    }
                    None => { }
                }
            }
            '=' => {
                match file_content.chars().nth(char_counter) {
                    Some(n) => {
                        if !should_skip_equal_sign {
                            // Look ahead if the last iteration was not a =
                            match file_content.chars().nth(char_counter + 1) {
                                Some(n_ahead_one) => {
                                    if n_ahead_one == '=' {
                                        println!("EQUAL_EQUAL == null");
                                        should_skip_equal_sign = true;
                                    } else {
                                        println!("EQUAL = null");
                                    }
                                }
                                None => {
                                  println!("EQUAL = null");
                                }
                            }
                        } else {
                            should_skip_equal_sign = false;
                        }
                    }
                    None => {}
                }
            }
            '\n' => { line_counter += 1 }
            // Unrecognized tokens
            '$' => {
                writeln!(io::stderr(), "[line {}] Error: Unexpected character: $", line_counter).unwrap();
                parse_error = 65;
            }
            '#' => {
                writeln!(io::stderr(), "[line {}] Error: Unexpected character: #", line_counter).unwrap();
                parse_error = 65;
            }
            '@' => {
                writeln!(io::stderr(), "[line {}] Error: Unexpected character: @", line_counter).unwrap();
                parse_error = 65;
            }
            '%' => {
                writeln!(io::stderr(), "[line {}] Error: Unexpected character: %", line_counter).unwrap();
                parse_error = 65;
            }
            _ => {}
        }
        char_counter += 1;
    }
    println!("EOF  null");
    match parse_error {
        0 => {}
        _ => { process::exit(parse_error) }
    }
}
