mod generator;

pub fn print_random_number() {
    let n = generator::gen_ran();
    println!("Random u8: {}", n);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn use_print_random_number() {
        print_random_number();
        assert!(true);
    }
}
