use std::io;

pub fn main() -> Result<(), io::Error> {
    println!("Hello, world!");
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn main_test() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
