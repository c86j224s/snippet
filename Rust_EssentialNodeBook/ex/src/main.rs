
//==============================================================================
struct Complex {
    real : f32,
    imag : f32
}

impl Complex {
    fn new(real: f32, imag: f32) -> Complex {
        Complex { real: real, imag: imag }
    }

    fn to_string(&self) -> String {
        format!("{}{:+}i", self.real, self.imag)
    }

    fn add(&self, v: Complex) -> Complex {
        Complex { real: self.real + v.real, imag: self.imag + v.imag }
    }

    fn times_ten(&mut self) {
        self.real *= 10.0;
        self.imag *= 10.0;
    }

    fn abs(&self) -> f32 {
        (self.real * self.real + self.imag * self.imag).sqrt()
    }
}

fn ex_5_1() {
    println!("\n#### ex_5_1() ####");

    let a = Complex::new(1.0, 2.0);
    println!("a = {}", a.to_string());

    let b = Complex::new(2.0, 4.0);
    println!("b = {}", b.to_string());

    let mut c = a.add(b);
    println!("c = {}", c.to_string());

    c.times_ten();
    println!("c*10 = {}", c.to_string());

    let d = Complex::new(-3.0, -4.0);
    println!("d = {}", d.to_string());
    println!("abs(d) = {}", d.abs());
}

//==============================================================================
trait Draw {
    fn draw(&self);
}

struct S1 {
    val : u32
}

struct S2 {
    val : f32
}

impl Draw for S1 {
    fn draw(&self) {
        println!("*** {} ***", self.val);
    }
}

impl Draw for S2 {
    fn draw(&self) {
        println!("*** {} ***", self.val);
    }
}

fn draw_object(obj : &Draw) {
    obj.draw();
}

fn ex_5_2() {
    println!("\n#### ex_5_2 ####");
    let s1 = S1 { val : 10 };
    draw_object(&s1);

    let s2 = S2 { val : 3.14 };
    draw_object(&s2);
}

//==============================================================================
#[derive(Eq)]
struct Position {
    x : u32,
    y : u32
}

impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn ex_5_3() {
    println!("\n#### ex_5_3 (Eq, PartialEq traits) ####");
    let pos1 = Position { x: 10, y: 20 };
    let pos2 = Position { x: 10, y: 20 };
    let pos3 = Position { x: 20, y: 40 };

    println!("pos1 == pos2 : {}", pos1 == pos2);
    println!("pos1 == pos3 : {}", pos1 == pos3);
    println!("pos1 != pos3 : {}", pos1 != pos3);
}

//==============================================================================
fn main() {
    ex_5_1();
    ex_5_2();
    ex_5_3();
}
