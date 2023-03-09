fn main() {
    let x = 5;
    println!("x before: {x}");
    {
        let y = &x + 1;
        println!("y inside: {y}");
    }
    println!("x after: {x}");
}
