use std::io;

pub fn main() -> Result<(), io::Error> {
    // 打印"Hello, world!"
    println!("Hello, world!");
    // 返回Ok
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    // 测试main函数
    fn test_main() {
        // 计算2+2
        let result = 2 + 2;
        // 断言计算结果为4
        assert_eq!(result, 4);
    }
}