pub fn run() {
    // mut 令 hello 为可变字符串
    let mut hello = String::from("Hello, ");
    println!("length: {}", hello.len());
    // push char
    hello.push('E');
    println!("{}", hello);

    // push string
    hello.push_str("ric");
    println!("{}", hello);

    // capacity in bytes
    println!("capacity: {}", hello.capacity());
    // check if empty
    println!("is empty: {}", hello.is_empty());

    // contains
    println!("contains: {}", hello.contains("Eric"));

    // replace
    println!("replace: {}", hello.replace("Eric", "Alice"));

    // loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // assertion testing，前面是期望值，后面是表达式
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
}
