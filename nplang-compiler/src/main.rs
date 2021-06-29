mod lexer;


fn main() {
    println!("Hello, world!");
    
    let mut program_buffer = lexer::types::ProgramBuffer::new_from_file(
        String::from("Cargo.toml")
    );

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
