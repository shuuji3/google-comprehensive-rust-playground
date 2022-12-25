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
    println!("t: '{:?}' and '{:?}'", t.0, t.1)
}
