fn main() {
    loop {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        let sum = s.split_whitespace()
            .filter_map(|c| c.parse().ok())
            .fold(0, |a, b: usize| a + b);
        println!("{}", sum)
    }
}
