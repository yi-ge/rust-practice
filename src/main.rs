fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn main() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
