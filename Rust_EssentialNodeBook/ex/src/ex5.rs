use std;

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
    println!("\n========== 1 ==========");

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
    println!("\n========== 2 ==========");
    let s1 = S1 { val : 10 };
    draw_object(&s1);

    let s2 = S2 { val : 3.14 };
    draw_object(&s2);
}

//==============================================================================
// Eq, Ord, Clone, Debug, Default and Add traits for struct

#[derive(Eq)]
#[derive(Default)]
struct Position {
    x : u32,
    y : u32
}

impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl std::cmp::Ord for Position {
    fn cmp(&self, other: &Position) -> std::cmp::Ordering {
        let abs = self.x * self.x + self.y * self.y;
        let abs_other = other.x * other.x + other.y * other.y;

        abs.cmp(&abs_other)
    }
}

impl std::cmp::PartialOrd for Position {
    fn partial_cmp(&self, other: &Position) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::clone::Clone for Position {
    fn clone(&self) -> Position {
        Position { x : self.x, y: self.y }
        // Or, derive Copy at Position and return *self here.
    }
}

impl std::ops::Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position { x: self.x + other.x, y: self.y + other.y }
    }
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Position{{ x:{}, y:{} }}", self.x, self.y)
    }
}

fn ex_5_3() {
    println!("\n========== 3 ==========");
    let pos1 = Position { x: 10, y: 20 };
    let pos2 = Position { x: 10, y: 20 };
    let pos3 = Position { x: 20, y: 40 };
    let pos4 = Position { x: 20, y: 10 };

    println!("pos1 == pos2 : {}", pos1 == pos2);
    println!("pos1 == pos3 : {}", pos1 == pos3);
    println!("pos1 != pos3 : {}", pos1 != pos3);
    println!("pos1 == pos4 : {}", pos1 == pos4);
    println!("pos1 < pos3 : {}", pos1 < pos3);
    println!("pos1 > pos3 : {}", pos1 > pos3);

    let pos5 = pos4.clone();
    println!("pos4 == pos5 : {}", pos4 > pos5);

    println!("pos1 + pos2 : {:?}", pos1 + pos2);

    let pos0 : Position = Default::default();
    println!("pos0 : {:?}", pos0);
}

//==============================================================================
// Default and Debug traits for enum

enum ButtonState {
    CLICKED,
    RELEASED
}

impl Default for ButtonState {
    fn default() -> ButtonState {
        ButtonState::CLICKED
    }
}

impl std::fmt::Debug for ButtonState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let state = match *self {
            ButtonState::CLICKED => "CLICKED",
            ButtonState::RELEASED => "RELEASED",
        };
        write!(f, "{}", state)
    }
}


fn ex_5_4() {
    println!("\n========== 4 ==========");
    let mut btn : ButtonState = Default::default();
    println!("btn = {:?}", btn);

    btn = ButtonState::RELEASED;
    println!("btn = {:?}", btn);
}


//==============================================================================
pub fn ex5() {
    println!("\n########## Example 5 ##########");
    ex_5_1();
    ex_5_2();
    ex_5_3();
    ex_5_4();
}
