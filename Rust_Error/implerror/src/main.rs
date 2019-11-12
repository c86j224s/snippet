
#[derive(Debug)]
enum MyError {
    IoError(std::io::Error),
    StrError(String),
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyError: {}", match *self {
            MyError::IoError(ref err) => format!("{}", err),
            MyError::StrError(ref err) => err.to_owned()
        })
    }
}

impl std::error::Error for MyError {
    fn description(&self) -> &str {
        match *self {
            MyError::IoError(ref err) => err.description(),
            MyError::StrError(ref err) => err.as_ref(),
        }
    }
}


fn main() {
    let e1 = MyError::IoError(std::io::Error::from(std::io::ErrorKind::Other));
    let e2 = MyError::StrError("second error".to_owned());

    println!("e1 = {:?}", e1);
    println!("e1 = {}", e1);

    println!("e2 = {:?}", e2);
    println!("e2 = {}", e2);

    println!("Hello, world!");
}
