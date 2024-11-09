pub fn tokenize(file_contents: String) -> i32 {
    let mut status_code: i32 = 0;
    let mut tokens: Vec<&str> = vec![];
    for (i, line) in file_contents.lines().enumerate() {
        for c in line.chars() {
            match c {
                '(' => tokens.push("LEFT_PAREN ( null"),
                ')' => tokens.push("RIGHT_PAREN ) null"),
                '{' => tokens.push(r#"LEFT_BRACE { null"#),
                '}' => tokens.push(r#"RIGHT_BRACE } null"#),
                '*' => tokens.push("STAR * null"),
                '.' => tokens.push("DOT . null"),
                ',' => tokens.push("COMMA , null"),
                '+' => tokens.push("PLUS + null"),
                '-' => tokens.push("MINUS - null"),
                ';' => tokens.push("SEMICOLON ; null"),
                '=' => {
                    let prev = tokens.pop();
                    if prev == Some("EQUAL = null") {
                        tokens.push("EQUAL_EQUAL == null");
                        continue;
                    } else if prev != None {
                        tokens.push(prev.unwrap());
                    }
                    tokens.push("EQUAL = null");
                }
                a => {
                    status_code = 65;
                    eprintln!("[line {}] Error: Unexpected character: {}", i + 1, a)
                }
            }
        }
    }
    tokens.push("EOF  null");

    for token in tokens {
        println!("{}", token);
    }
    status_code
}
