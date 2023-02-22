use core::slice;
use std::f64::consts::PI;

use chapter_nineteen::HelloMacro;
use hello_macro_derive::HelloMacro;

fn main() {
    uses_unsafe_rust();

    uses_unsafe_to_call_external_c_function();

    uses_macro();

    uses_proc_derive_macro();
}

fn uses_unsafe_rust() {
    println!("Using unsafe rust");

    let mut xs = [1, 2, 3, 4, 5, 6];
    let (x1, x2) = split_at_mut(&mut xs, 3);
    println!("{x1:?} + {x2:?}");
}

// defining external code that follows the C ABI
extern "C" {
    // in src/c_module.c
    fn print_and_return(x: i32) -> i32;
    // from the C std lib
    fn abs(input: i32) -> i32;
}

fn uses_unsafe_to_call_external_c_function() {
    unsafe {
        let x = print_and_return(5);
        println!("Got {x} from C code");

        let absv = abs(-3);
        println!("Absolute value {absv}");
    }
}

fn split_at_mut<T>(values: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    let len = values.len();

    assert!(
        mid <= len,
        "mid must be less than or equal to the length of values"
    );

    // naive implementation that doesn't compile:
    // (&mut values[..mid], &mut values[mid..]) <- cannot borrow as mutable more than once
    // the compiler has no way of knowing that both slides do not overlap thus NOT actually creating a memory safety problem
    // but we can guarantee that ourselves:

    let ptr = values.as_mut_ptr(); // mutable raw pointer to the beginning of the slice
    unsafe {
        // we manually create the slices by using the non-overlaping regions of memory
        (
            // slice: (ptr)-> [ mem::size_of(T) * mid ]
            slice::from_raw_parts_mut(ptr, mid),
            // slice: (ptr + mid * mem::size_of(T))-> [mem::size_of(T) * (lem - mid)]
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

#[macro_export]
macro_rules! hashmap {
    // counting expressions: https://danielkeep.github.io/tlborm/book/blk-counting.html
    // counting using the same macro: https://docs.rs/maplit/1.0.2/src/maplit/lib.rs.html#1-324
    (@single $_t:tt $sub:expr) => { $sub };
    (@count $($tts:tt)*) => {<[()]>::len(&[$(hashmap!(@single $tts ())),*])};

    ( $( $key:expr  => $val:expr ),* ) => {
        {
            let cap = hashmap!(@count $($key)*);
            let mut hash_map = std::collections::HashMap::with_capacity(cap);
            $(
                hash_map.insert($key, $val);
            )*
            hash_map
        }
    };
}

fn uses_macro() {
    println!("Using declarative macro");

    let m1 = hashmap!("a" => 1, "b" => 2);
    println!("{m1:?}");

    let m2 = hashmap!(
        1 => None,
        2 => Some(1.5),
        3 => Some(PI)
    );

    println!("{m2:?}");
}

#[derive(HelloMacro)]
struct Pancakes;

fn uses_proc_derive_macro() {
    println!("Using procedural derive macro");

    Pancakes::hello_macro();
}
