pub fn run() {
    // 结构体中的泛型
    #[derive(Debug)]
    struct Point1<T> {
        x: T,
        y: T,
    }
    impl<T> Point1<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }
    let p1 = Point1 { x: 10, y: 20 };
    let p2 = Point1 { x: "10", y: "20" };
    println!("{:#?}, {:#?}", p1, p2);
    println!("p.x = {}", p1.x());

    // 多参数
    // #[derive(Debug)]
    // struct Point<T, U> {
    //     x: T,
    //     y: U,
    // }

    // let p3 = Point { x: 10, y: 20.1 };
    // let p4 = Point { x: "10", y: 20.10 };

    // 枚举中的泛型
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
}
