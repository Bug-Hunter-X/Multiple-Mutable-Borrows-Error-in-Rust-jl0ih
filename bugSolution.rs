fn main() {
    let mut x = 5;
    { // Use a scope for the first borrow
        let y = &mut x; 
        *y += 1;
    }
    { // Use a new scope for the second borrow
        let z = &mut x;
        *z += 1;
    }
    println!("x = {}", x);
}
//Another solution using clone
fn main() {
    let mut x = 5;
    let mut y = x;
    let mut z = x;
    y += 1;
    z += 1;
    x = y + z -10;
    println!("x = {}", x);
} 