// It's now hardcoded how many numbers fibonacci_array() calculates
const FIBONACCI_NUMS: usize = 100;

// Inspect the resulting assembly using:
// cargo asm --rust --bin nth_branchless nth_fibonacci
#[inline(never)] // so that we can easily view the assembly
fn nth_fibonacci(n: usize, fibonacci: &[u64]) -> u64 {
    // Instead of panicking, return a bogus value.
    // This this gets optimized into a conditional move
    // instead of a branch instruction, which is
    // slower than a branch if the value is always in bounds
    // because the branch is perfectly predictable,
    // but faster if you query values both in and out of bounds
    // because it avoids costly branch misprediction.
    *fibonacci.get(n).unwrap_or(&0)
}

fn fibonacci_vec() -> Vec<u64> {
    let length = FIBONACCI_NUMS;
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
    // read the length at runtime so that the compiler can't just precompute the result
    let arg = std::env::args().nth(1).expect("Please specify the number to look up");
    let index: usize = arg.parse().expect("Lookup index is not a number!");
    // generate the lookup table
    let fibonacci = fibonacci_vec();
    // actually call the function we care about
    let result = nth_fibonacci(index, &fibonacci);
    // and print the result so that the compiler doesn't remove it as dead code
    println!("{:?}", result);
}
