mod lexer;


fn main() {
    println!("Hello, world!");

    let mut l = lexer::ProgramLexer::new_from_file(
        String::from("test.np")
    );

    l.dump_tokens(String::from("dump.txt"));
}
