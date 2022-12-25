// It's now hardcoded how many numbers fibonacci_array() calculates
const FIBONACCI_NUMS: usize = 100;
// round that number up to a power of two
// to make our custom bounds checks really cheap
const LOOKUP_TABLE_SIZE: usize = FIBONACCI_NUMS.next_power_of_two();

// Inspect the resulting assembly using:
// cargo asm --rust --bin nth_errorless_stack nth_fibonacci
#[inline(never)] // so that we can easily view the assembly
fn nth_fibonacci(n: usize, fibonacci: &[u64; LOOKUP_TABLE_SIZE]) -> u64 {
    // we're going to blithely ignore any errors further on
    // to squeeze out every last bit of performance,
    // but that's no excuse not to sanity-check in tests
    debug_assert!(n < FIBONACCI_NUMS);
    // remainder operator % is expensive in the general case,
    // but for a constant equal to power of two optimizes into
    // a bitwise AND, which is very cheap.
    // However:
    // out-of-bounds accesses now return garbage instead of panicking!
    fibonacci[n % LOOKUP_TABLE_SIZE]
}

fn fibonacci_array() -> [u64; LOOKUP_TABLE_SIZE] {
    let length = FIBONACCI_NUMS;
    // The array is allocated on the stack.
    // The syntax happens to be more terse
    // than when doing the same on the heap,
    // but this will overflow the stack
    // given very large lookup table sizes.
    let mut fib = [0; LOOKUP_TABLE_SIZE];
    if length > 1 {
        fib[1] = 1;
    }
    if length > 2 {
        let mut grandparent = 0;
        let mut parent = 1;
        for val in &mut fib[2..FIBONACCI_NUMS] {
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
    let fibonacci = fibonacci_array();
    // actually call the function we care about
    let result = nth_fibonacci(index, &fibonacci);
    // and print the result so that the compiler doesn't remove it as dead code
    println!("{:?}", result);
}