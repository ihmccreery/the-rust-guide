fn main() {
    let x: int = 4;

    let y = if x == 5 { 10i } else { 5i };

    println!("number: {}", add_one(y));
}

/// `add_one` adds one to the given int
/// 
/// # Arguments
/// 
/// * `x` - The integer to which to add one.
/// 
/// # Example
/// 
/// ```rust
/// let x = 5i;
/// x = add_one(x); // x is now 6i.
/// ```
fn add_one(x: int) -> int {
    x + 1
}
