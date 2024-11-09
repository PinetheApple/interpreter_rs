pub fn tokenize(file_contents: String) {
    for line in file_contents.lines() {
        for c in line.chars() {
            match c {
                '(' => println!("LEFT_PAREN ( null"),
                ')' => println!("RIGHT_PAREN ) null"),
                '{' => println!("LEFT_BRACE {{ null"),
                '}' => println!("RIGHT_BRACE }} null"),
                _ => {}
            }
        }
    }
    println!("EOF  null");
}
