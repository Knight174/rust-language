pub fn run() {
    let rect = Rectangle {
        width: 10,
        height: 20,
    };

    println!("{}", rect.area());
    println!("{:#?}", rect);
    println!("{:?}", rect);

    let sq = Rectangle::square(3);
    println!("{:#?}", sq);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// 关联方法
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
