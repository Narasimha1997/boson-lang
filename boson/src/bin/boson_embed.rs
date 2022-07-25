use boson::api::BosonLang;

pub fn main() {
    let result = BosonLang::eval_buffer("10 + 3 / 2".as_bytes().to_vec());

    println!("eval result: {:?}", result);
}
