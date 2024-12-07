use std::{
    fs::{read_to_string, File},
    io::{self, BufRead, BufReader},
};

use parse_mul::{parse_mul, parse_mul_disable};

pub mod parse_mul;

fn main() -> io::Result<()> {
    let reader = BufReader::new(File::open("input.txt")?);
    let result: usize = reader.lines().map(|l| parse_mul(&l.unwrap())).sum();
    println!("Sum of all multiplications: {}", result);
    let content = read_to_string("input.txt").unwrap();
    let result = parse_mul_disable(&content);
    println!("Sum without disabled ones: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let reader = BufReader::new(File::open("input.txt").unwrap());
        let result: usize = reader.lines().map(|l| parse_mul(&l.unwrap())).sum();
        assert_eq!(result, 175015740);
        let content = read_to_string("input.txt").unwrap();
        let result = parse_mul_disable(&content);
        assert_eq!(result, 112272912);
    }
}
