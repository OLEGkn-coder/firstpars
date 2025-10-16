use firstpars::list_parser;
fn main() {
    let one = "[2, 1, 5, 6,7 ]";
    match list_parser::list(one) {
        Ok(n) => print!("Pars: {:?}", n),
        Err(e) => eprint!("Er: {}", e),
    }
}
