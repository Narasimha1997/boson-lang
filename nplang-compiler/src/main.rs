mod lexer;


use std::fs;

fn main() {
    println!("Hello, world!");
    let data = fs::read_to_string("Cargo.toml").expect(
        "File not found!"
    );

    let bytes = data.as_bytes().iter().cloned().collect();
    let mut program_buffer = lexer::types::new_program_buffer(bytes);

    let mut ch: u8;

    loop {
        ch = program_buffer.next_char();
        if ch == 0x00 {
            println!("End of buffer");
            break;
        }
        println!("{}", ch as char);
    }
}
