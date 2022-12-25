fn main() {
    println!("Hello, 🌎!");

    let mut x: i32 = 6;
    print!("{x}");
    while x != 1 {
        if x % 2 == 0 {
            x /= 2;
        } else {
            x = 3 * x + 1;
        }
        print!(" -> {x}");
    }
    println!();

    let mut a: [i8; 10] = [42; 10];
    a[5] = 0;
    println!("a: {:?}", a);

    let t: (i8, bool) = (7, true);
    println!("t: '{:?}' and '{:?}'", t.0, t.1);

    let mut x: i32 = 10;
    let ref_x: &mut i32 = &mut x;
    *ref_x = 20;
    println!("x: {x}");

    let a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");

    let s: &[i32] = &a[2..4];
    println!("s: {s:?}");

    let s1: &str = "World";
    println!("s1: {s1:?}");

    let mut s2: String = String::from("Hello ");
    println!("s2: {s2:?}");
    s2.push_str(s1);
    println!("s2: {s2:?}");

    fizzbuzz_to(20);

    let mut rect = Rectangle { width: 10, height: 5 };
    println!("old area: {}", rect.area());
    rect.inc_width(5);
    println!("old area: {}", rect.area());

    let id = std::process::id();
    println!("id: {id}");

    println!("coin toss: {}", pick_one("heads", "tails"));
    println!("prize: {}", pick_one(500, 1000));
}

fn pick_one<T>(a: T, b: T) -> T {
    if std::process::id() % 2 == 0 { a } else { b }
}

fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fuzzbuzz(n)
    }
}

fn fuzzbuzz(n: u32) {
    match (is_divisible_by(n, 3), is_divisible_by(n, 5)) {
        (true, true) => println!("fizzbuzz"),
        (true, false) => println!("fizz"),
        (false, true) => println!("buzz"),
        (false, false) => println!("{n}"),
    }
}

fn is_divisible_by(n: u32, div: u32) -> bool {
    return n % div == 0;
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn inc_width(&mut self, delta: u32) {
        self.width += delta;
    }
}

