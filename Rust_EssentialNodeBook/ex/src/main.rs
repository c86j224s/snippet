
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


fn main() {
    ex_5_1();
}
