mod lexer;

use std::str;

fn main() {
    println!("Hello, world!");

    let mut chunked_reader = lexer::types::new_chunked_reader(
        String::from("Cargo.toml"), 10
    );

    while !chunked_reader.is_end() {
        let lex_buffer = chunked_reader.next();
        match lex_buffer {
            Some(lb) => {
                println!("Buffer has data");
                println!("{}", str::from_utf8(&lb.buffer).expect("Failed do utf-8 decode"));
            },
            None => {
                println!("End of buffer!");
            }
        }
    }
}
