mod lexer;


fn main() {
    println!("Hello, world!");

    let mut l = lexer::ProgramLexer::new_from_file(
        String::from("test.np")
    );

    let mut token = l.next_token();
    println!("{:?}", token);
    token = l.next_token();
    println!("{:?}", token);
    token = l.next_token();
    println!("{:?}", token);
    token = l.next_token();
    println!("{:?}", token);
    token = l.next_token();
    println!("{:?}", token);
    token = l.next_token();
    println!("{:?}", token);
    token = l.next_token();
    println!("{:?}", token);
    token = l.next_token();
    println!("{:?}", token);
    token = l.next_token();
    println!("{:?}", token);
    token = l.next_token();
    println!("{:?}", token);
    token = l.next_token();
    println!("{:?}", token);
    token = l.next_token();
    println!("{:?}", token);
    token = l.next_token();
    println!("{:?}", token);
    token = l.next_token();
    println!("{:?}", token);
    token = l.next_token();
    println!("{:?}", token);
    token = l.next_token();
    println!("{:?}", token);
    token = l.next_token();
    println!("{:?}", token);
    token = l.next_token();
    println!("{:?}", token);
    token = l.next_token();
    println!("{:?}", token);
}
