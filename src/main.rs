fn check_keyword(s: &str) -> u32 {
    if s == "return" || s == "int" || s == "if" {
        return 1;
    }
    0
}
fn lexer(s: String) {
    for i in s.split(' ') {
        let token = i.to_string();
        if token.chars().all(char::is_alphabetic) && check_keyword(&token) == 1 {
            println!("Keyword: {token}");
        } else if token.chars().all(char::is_alphabetic) && check_keyword(&token) == 0 {
            println!("Identifier: {token}");
        } else if token.chars().all(|c| c.is_ascii_digit()) {
            println!("Number: {token}");
        } else if token == "+" || token == "-" {
            println!("Operator: {token}");
        }
    }
}

fn main() {
    let code: String = String::from("int x = 10 + 20");
    lexer(code);
}
