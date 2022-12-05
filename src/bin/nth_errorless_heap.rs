// It's now hardcoded how many numbers fibonacci_array() calculates
const FIBONACCI_NUMS: usize = 100;
// round that number up to a power of two
// to make our custom bounds checks really cheap
const LOOKUP_TABLE_SIZE: usize = FIBONACCI_NUMS.next_power_of_two();

#[inline(never)] // so that we can easily view the assembly
fn nth_fibonacci(n: usize, fibonacci: &[u64; LOOKUP_TABLE_SIZE]) -> u64 {
    // we're going to blithely ignore any errors further on
    // to squeeze out every last bit of performance,
    // but that's no excuse not to sanity-check in tests
    debug_assert!(n < FIBONACCI_NUMS);
    // remainder operator % is expensive in the general case,
    // but for a constant equal to power of two optimizes into
    // a bitwise AND, which is very cheap.
    // However,
    // out-of-bounds accesses now return garbage instead of panicking!
    fibonacci[n % LOOKUP_TABLE_SIZE]
}

fn fibonacci_vec() -> Box<[u64; LOOKUP_TABLE_SIZE]> {
    let length = FIBONACCI_NUMS; // hardcoded
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

    // Pad the Vec with zeroes to the next power of two
    fib.resize(LOOKUP_TABLE_SIZE, 0);
    // Convert the Vec to an owned slice, its size is now unchanging
    let fib_slice: Box<[u64]> = fib.into_boxed_slice();
    // Coerce the slice to a fixed-size type, this encodes the size in the type system
    let fib_fixed_slice: Box<[u64; LOOKUP_TABLE_SIZE]> = fib_slice.try_into().unwrap();

    fib_fixed_slice
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