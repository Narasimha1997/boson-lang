
fn get_n() -> Result<u8, String> {
    return Ok(10);
}

fn main() {
    let n1: u8 = 10;

    let result = get_n().ok().map(
        |n| n1.checked_mul(10).map(|res| res / n)
    );
    println!("{:?}", result);
}