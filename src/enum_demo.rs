use std::fs::File;

pub fn run() {
    /*
     * enum Option<T> {
     *   Some<T>,
     *   None
     * }
     */
    let a = Some(5);
    match a {
        Some(value) => println!("The value is: {}", value),
        None => println!("The value is None"),
    }

    let b = Some(10).unwrap();
    println!("The value is: {}", b);

    /*
     * enum Result<T, E> {
     *   Ok<T>,
     *   Error(#)
     * }
     */
    // let f1 = File::open("hello.txt").expect("open file error");
    // println!("{:#?}", f1);

    let f2 = File::open("hello.txt");
    match f2 {
        Ok(file) => println!("{:#?}", file),
        Err(error) => {
            panic!("open file error: {:#?}", error);
        }
    }
}
