use std::io;

pub fn main() -> Result<(), io::Error> {
    println!("Hello, world!");
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_main() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
