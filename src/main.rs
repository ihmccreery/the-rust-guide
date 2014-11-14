fn main() {
    let x: int = 4;

    let y = if x == 5 { 10i } else { 5i };

    print_number(add_one(y));
}

fn print_number(x: int) {
    println!("number: {}", x);
}

fn add_one(x: int) -> int {
    if x > 4 { return x };

    x + 1
}
