// Inspect the resulting assembly using:
// cargo asm --rust --bin comparison_realistic is_fibonacci
#[inline(never)] // so that we can easily view the assembly
fn is_fibonacci(input: &[u64], fibonacci: &[u64]) -> bool {
    input == &fibonacci[..input.len()]
}

fn fibonacci_vec(length: usize) -> Vec<u64> {
    let mut fib = vec![0; length];
    if length > 1 {
        fib[1] = 1;
    }
    if length > 2 {
        let mut grandparent = 0;
        let mut parent = 1;
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
    let arg = std::env::args().nth(1).expect("Please specify precomputed length");
    let length: usize = arg.parse().expect("Precomputed length is not a number!");
    let arg = std::env::args().nth(2).expect("Please specify test length");
    let test_len: usize = arg.parse().expect("Test length is not a number!");
    // generate the lookup table
    let fibonacci = fibonacci_vec(length);
    // generate the array we're going to test - whether it's Fibonacci or not
    let input = fibonacci_vec(test_len);
    // actually call the function we care about
    let result = is_fibonacci(&input, &fibonacci);
    // and print the result so that the compiler doesn't remove it as dead code
    println!("{:?}", result);
}