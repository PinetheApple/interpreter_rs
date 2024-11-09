pub fn tokenize(file_contents: String) -> i32 {
    let mut status_code: i32 = 0;
    for line in file_contents.lines() {
        for (i, c) in line.chars().enumerate() {
            match c {
                '(' => println!("LEFT_PAREN ( null"),
                ')' => println!("RIGHT_PAREN ) null"),
                '{' => println!("LEFT_BRACE {{ null"),
                '}' => println!("RIGHT_BRACE }} null"),
                '*' => println!("STAR * null"),
                '.' => println!("DOT . null"),
                ',' => println!("COMMA , null"),
                '+' => println!("PLUS + null"),
                '-' => println!("MINUS - null"),
                ';' => println!("SEMICOLON ; null"),
                a => {
                    status_code = 65;
                    eprintln!("[line {}] Error: Unexpected character: {}", i + 1, a)
                }
            }
        }
    }
    println!("EOF  null");
    status_code
}
