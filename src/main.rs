fn check_keyword(s: &str) -> u32 {
    if s == "return" || s == "int" || s == "if" {
        return 1;
    }
    0
}
fn lexer(s: &String) {
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
fn check_operator(op: char) -> bool {

    op == '+' || op == '-' || op == '*' || op == '/'

}
fn check_declared(s: &str, vars: &[[char; 10]; 2], size: usize) -> bool {
    for var in vars.iter().take(size) {
        let var_string: String = var.iter()
            .take_while(|&&c| c != '\0')
            .collect();
        if s == var_string {
            return true;
        }
    }
    false

}
fn semantic_analyzer(statement: &str, vars: &[[char; 10]; 2]) {
    let variable = statement
        .split_whitespace()
        .next()
        .unwrap_or("");

    if !check_declared(variable, vars, 2) {
        println!("Semantic Error: Variable '{}' not declared", variable);
    } else {
        println!("No semantic errors.");}
}
fn check_valid_expression(x: &String) -> bool {
  let s: String =  x.chars().filter(|c| !c.is_whitespace()).collect();
    let mut i  = 0;
    if !s.chars().nth(i).unwrap().is_alphabetic() {
        println!("Syntax Error Expected Identifier: {s}");
        return false;
    }
    i = i + 1;
    if s.chars().nth(i) != Some('=') {
        println!("Syntax Error Expected Operator: {s}");
        return false;
    }

    i = i + 1;

   while i < s.len() -1 {
       if !s.chars().nth(i).unwrap().is_numeric() && !check_operator(s.chars().nth(i).unwrap()) {
           println!("Syntax Error Invalid Expression: {s}");
           return false;
       }
       i = i + 1;
   }
    if s.chars().nth(i) != Some(';') {
        println!("Syntax Error Expected ;: {s}");
        return false;
    }
    println!("Valid Syntax");
   true
}
fn generate_intermediate_code(statement: &str) {
    let tokens: Vec<&str> = statement.split_whitespace().collect();

    if tokens.len() != 5 || tokens[1] != "=" {
        println!("Invalid statement format");
        return;
    }

    let variable = tokens[0];
    let operand1: i32 = tokens[2].parse().unwrap_or(0);
    let operator = tokens[3].chars().next().unwrap_or('?');
    let operand2: i32 = tokens[4].parse().unwrap_or(0);

    println!("t1 = {operand1}");
    println!("t2 = {operand2}");
    println!("t3 = t1 {operator} t2");
    println!("{variable} = t3");
}
fn generate_machine_code(statement: &str) {
    let tokens: Vec<&str> = statement.split_whitespace().collect();

    let variable = tokens[0];
    let operand1: i32 = tokens[2].parse().unwrap_or(0);
    let operator = tokens[3].chars().next().unwrap_or('?');
    let operand2: i32 = tokens[4].parse().unwrap_or(0);

    println!("MOV R1, #{operand1}");  // Load operand1 into R1
    println!("MOV R2, #{operand2}");  // Load operand2 into R2
    println!("{operator} R1, R2");    // Perform the operation
    println!("MOV {variable}, R1");   // Store result in the variable
}

fn main()
{
    let mut vars: [[char; 10]; 2] = [['\0'; 10]; 2];
    vars[0][0] = 'x';
    vars[1][0] = 'y';
    let code: String = String::from("int x = 10 + 20");
    let statement1: String = String::from("x = 10 + 5;");
    let statement2: String = String::from("x = 10 + y;");
    lexer(&code);
    check_valid_expression(&statement1);
    semantic_analyzer(&statement2,&vars);
    generate_intermediate_code(&statement1);
    generate_machine_code(&statement1);
}

