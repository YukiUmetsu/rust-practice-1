use std::ops::Add;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

struct NoneDev {
    isdev: bool
}
struct RustDev {
    awesome: bool
}

struct PHPDev {
    awesome: bool
}

trait Developer {
    fn new(awesome: bool) -> Self;
    fn language(&self) -> &str;
    // default implementation. Not required to implement this fun.
    fn say_hello(&self) { println!("Hello") }
}

impl Developer for RustDev {
    fn new(awesome: bool) -> Self {
        RustDev { awesome: awesome }
    }

    fn language(&self) -> &str {
        "Rust"
    }

    fn say_hello(&self) {
        println!("hey from rust dev")
    }
}

impl Developer for PHPDev {
    fn new(awesome: bool) -> Self {
        PHPDev { awesome: awesome }
    }


    fn language(&self) -> &str {
        "php 7.0"
    }


    fn say_hello(&self) {
        println!("hey from php dev")
    }
}

// Generic function
fn can_code<T: Developer>(d: T) {
    println!("I can code in {}", d.language());
}

fn main() {
    let r = RustDev::new(true);
    let p = PHPDev::new(false);
    let n = NoneDev{isdev: false};
    println!("{}", r.language());
    r.say_hello();

    println!("{}", p.language());
    p.say_hello();

    // use generic function
    can_code(r);
    can_code(p);
    // can_code(n); this fails because  the trait `Developer` is not implemented for `NoneDev`

    println!("\n-----------------\n");

    let p1 = Point{ x: 1.3, y: 4.6 };
    let p2 = Point{ x: 4.7, y: 1.4 };

    println!("{:?}", p1 + p2);
}