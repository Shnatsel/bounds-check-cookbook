// Inspect the resulting assembly using:
// cargo asm --rust --bin comparison_split elements_are_equal
#[inline(never)] // so that we can easily view the assembly
fn elements_are_equal(slice1: &[u64], slice2: &[u64], index: usize) -> bool {
    // This is now a standalone function, and there is no constraint on 
    // how it can be called! Bounds checks are back!
    slice1[index] == slice2[index]
}

fn is_fibonacci(input: &[u64], fibonacci: &[u64]) -> bool {
    // Cut off one slice up to the length of the other,
    // so that the compiler knows the lengths are the same
    let fibonacci = &fibonacci[..input.len()];

    for i in 0..input.len() {
        if elements_are_equal(input, fibonacci, i) {
            return false;
        }
    }
    true
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