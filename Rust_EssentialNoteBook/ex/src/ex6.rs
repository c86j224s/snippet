use std;

//==============================================================================
// 라이프타임은 최소 해당 블록 안 쪽이라고 볼 수 있음.
// 구조체 내부의 참조 타입 멤버 변수는 lifetime ellision이 불가능하고, 반드시 수명을
// 명시해야 함.

// 기본적인 T의 대입 연산은 move이나, Copy 트레잇을 구현하면 암시적인 복사가 가능하다.

// Clone 트레잇은 명시적인 객체 복제(clone)를 구현할 수 있게 한다. 

// &T 혹은 &mut T를 pointer 혹은 refrence라고 한다. *val 과 같이 dereference 할 수 있다.

// borrow는 mutable reference로는 한번에 하나씩만 빌릴 수 있다.
// immutable reference는 한번에 여러개씩 빌릴 수 있다.
// 다만 mutable reference와 immutable reference를 동시에 빌릴 수는 없다.

// *T 혹은 *const T는 raw pointer라고 하며, 그 주소값 자체를 얻을 수 있다.

// pattern matching 내에서의 destructing을 할 때는 ref 라는 키워드를 쓸 수 있다.

// Box는 이동연산만을 지원하는 힙 메모리 할당 방식이다.
// 대부분의 경우 Box대신 일반적인 레퍼런스를 사용하는 것이 더 낫지만,
// 유일하게 한가지 케이스, 구조체가 자기 자신의 재귀 참조를 필요로 하는 경우
// 목적에 부합한다.
// 예시 : https://doc.rust-lang.org/std/boxed/

// Rc는 immutable value를 reference counting 방식으로 관리할 수 있도록 하는
// 기능이다. thread safety 보장까지 필요하다면 Arc를 이용한다.

// Cell은 reference counting 방식은 아니지만 (소유자는 항상 1개이다)
// 여러 cell에서 mutability를 보장해주는 기능이다. 
// thread safety는 보장하지 않을 것 같다. (추측)

// Arc는 Rc의 thread safety를 보장하는 버전이라고 한다.


// 라이브러리들까지 포함해, 포인터 타입은 다음과 같은 종류들이 있다. (정리)
// &T, &mut T, Box<T>, Cell<T>, Rc<T>, Arc<T>, *const T, *mut T

// arachneng님의 '러스트 타입 주기율표'
// http://cosmic.mearie.org/2014/01/periodic-table-of-rust-types/
// 한글 해설 : http://j.mearie.org/post/73635438329/explaining-the-periodic-table-of-rust-types

struct Alien {
    planet: String,
    n_tentacles: u32
}

fn err_cases() {
    //let klaatu = Alien { planet: "Pluto".to_string(), n_tentacles: 8 };

    //let klaatuc = klaatu;

    //let kl2 = &klaatu;

    // kl2는 mutable reference로 klaatu를 빌리지 않았기 때문에 변경을 할 수 없다.
    // 사실, klaatu 자체가 불변이기 때문에 klaatu 자체로도 변경할 수 없다.
    //kl2.n_tentacles = 9;

    //klaatu.n_tentacles = 10;
}

fn immut_to_mut() {
    println!("\n===== 불변 변수에서 가변 변수로 소유권 이전 후 변경 =====");
    let a = Alien { planet: "Klaatu".to_string(), n_tentacles: 32 };

    let mut b = a;

    //println!("{}, {}", a.planet, a.n_tentacles);

    
    b.n_tentacles = 42;

    println!("{}, {}", b.planet, b.n_tentacles);
}

fn grow_a_tentacle(alien : &mut Alien) {
    alien.n_tentacles += 1;
}

fn grow() {
    println!("\n===== 외계인 빌려주기 =====");
    let mut klaatu = Alien { planet: "Europa".to_string(), n_tentacles: 8 };

    grow_a_tentacle(&mut klaatu);
    grow_a_tentacle(&mut klaatu);

    println!("{}, {}", klaatu.planet, klaatu.n_tentacles);
}

fn reference() {
    println!("\n===== reference 실험 =====");

    let mut a = 42;

    {
        let pa1 = &a;
        let pa2 = &a;

        println!("{}", *pa1);
        println!("{}", *pa2);
    }

    {
        let pa3 = &mut a;

        println!("{}", *pa3);
    }
}

fn pattern_matching () {
    println!("\n===== pattern matching =====");

    let a = 42;
    match a {
        ref r => println!("ref to {}", r),
            r => println!("val to {}", r),
    }
}

fn raw_pointer() {
    println!("\n===== raw pointer 실험 =====");

    let a = 42;
    let ptr_a : *const u32 = &a; // same as let ptr_a = &a as *const u32;

    println!("{:p}", &a);
    println!("{:?}", ptr_a);
    unsafe {
        println!("{}", *ptr_a);
    }
}

fn box_example() {
    println!("\n===== box 실험 =====");

    let mut a = Box::new(32);
    *a = 42;

    println!("a = {}", a);

    let mut b = a;  // b is Box<{integer}>
    //  println!("a = {}", a);      // error.
    println!("b = {}", b);

    let c = &*b;    // c is &{integer}
    println!("c = {}", c);
}

#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil
}

fn box2_example() {
    println!("\n===== box 실험 2 =====");

    let l = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    println!("{:?}", l);
}

fn rc_example() {
    println!("\n===== rc 실험 =====");

    let a = std::rc::Rc::new(30);
    println!("a = {}", a);

    let b = a.clone();
    println!("a = {}, b = {}", a, b);
}

fn cell_example() {
    println!("\n===== cell 실험 =====");

    let a = std::cell::Cell::new(30);
    println!("a = {:?}", a);

    let b = &a;
    println!("a = {:?}, b = {:?}", a, b);

    a.set(42);
    println!("a = {:?}, b = {:?}", a, b);

    /* error below

    let d = {
        let c = std::cell::Cell::new(10);

        println!("c = {:?}", c);

        c.set(20);

        println!("c = {:?}", c);

        &c
    };

    println!("d = {:?}", d);
    */
}

pub fn ex6() {
    println!("\n##### 소유권과 빌림 #####"); 
    err_cases();
    immut_to_mut();
    grow();
    reference();
    pattern_matching();
    raw_pointer();
    box_example();
    box2_example();
    rc_example();
    cell_example();
}
