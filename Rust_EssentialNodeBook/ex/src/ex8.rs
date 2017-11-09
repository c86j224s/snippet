macro_rules! mac2 {
    ($e:expr) => (($e) * 3)
}

macro_rules! mac3 {
    ($i:ident) => (let $i = 42);
}

macro_rules! mac4 {
    ($e:expr) => (println!("start - {} - end", $e));
}

pub fn ex8() {
    println!("triple of {} : {}", 3, mac2!(3));
    
    mac3!(hello);
    println!("mac3 result : {}", hello);

    mac4!("Where am I?");
}
