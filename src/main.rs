fn main() {
    match transactiondecoder::run(transactiondecoder::get_arg()) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("{}", e),
    }
}