#[derive(Debug)]
pub enum EnumTest {
    None,
    NODE(String, Box<String>),
}

#[test]
fn test_enum() {
    println!("{:?}", EnumTest::None);
    println!(
        "{:?}",
        EnumTest::NODE(String::from("abc"), Box::new(String::from("def")))
    );

    // 解构
    let bb = Box::new(String::from("def"));
    let a = EnumTest::NODE(String::from("abc"), bb);
    let res = match a {
        EnumTest::None => None,
        EnumTest::NODE(a, b) => {
            println!("a: {:?}", a);
            println!("b: {:?}", b);
            // println!("bb == b: {:?}", bb == b);
            Some(b)
        }
    };

    assert_eq!(*(res.unwrap()), String::from("def"));
}
