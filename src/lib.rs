pub fn tokenize(file_contents: String) {
    for line in file_contents.lines() {
        for c in line.chars() {
            match c {
                '(' => println!("LEFT_PAREN ( null"),
                ')' => println!("RIGHT_PAREN ) null"),
                _ => {}
            }
        }
    }
    println!("EOF  null");
}
