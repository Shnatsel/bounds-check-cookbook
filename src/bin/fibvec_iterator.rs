#[inline(never)] // so that we can easily view the assembly
fn fibonacci_vec(length: usize) -> Vec<u64> {
    let mut fib = vec![0; length];
    if length > 1 {
        fib[1] = 1;
    }
    if length > 2 {
        let mut grandparent = 0;
        let mut parent = 1;
        // this is an iterator; it completely avoids bounds checks
        for val in &mut fib[2..] {
            let current = grandparent + parent;
            *val = current;
            grandparent = parent;
            parent = current;
        }
    }

    fib
}

pub fn main() {
    // read the length at runtime so that the compiler can't just precompute Fibonacci
    let arg = std::env::args().nth(1).expect("Please specify length");
    let length: usize = arg.parse().expect("That's not a number!");
    // actually call the function we care about
    let fibonacci = fibonacci_vec(length);
    // and print the result so that the compiler doesn't remove it as dead code
    println!("{:?}", fibonacci.last());
}