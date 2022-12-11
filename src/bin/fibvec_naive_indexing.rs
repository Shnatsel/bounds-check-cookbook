// Inspect the resulting assembly using:
// cargo asm --rust --bin fibvec_naive_indexing fibonacci_vec
#[inline(never)] // so that we can easily view the assembly
fn fibonacci_vec(length: usize) -> Vec<u64> {
    let mut fib = vec![0; length];
    
    if length > 1 {
        fib[1] = 1; // indexing! bounds check! But just once, so it's okay
    }
    if length > 2 {
        for i in 2..length {
            fib[i] = fib[i-1] + fib[i-2]; // indexing in a loop! Oh no!
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