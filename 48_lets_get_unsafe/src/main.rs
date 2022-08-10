use std::slice;

fn main() {
    dereferencing_raw_pointers();
    calling_unsafe_functions();
    encapsulating_unsafe_in_safe();
    accessing_and_modifying_static_variables()
}
fn accessing_and_modifying_static_variables() {
    static mut COUNTER: u32 = 0;

    fn add_to_counter(num: u32) {
        unsafe {
            COUNTER += num;
        }
    }

    add_to_counter(3);
    unsafe {
        println!("UNSAFE : Counter is now {}", COUNTER);
    }
}

fn encapsulating_unsafe_in_safe() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    let (c, d) = custom_split_at_mut(r, 3);

    assert_eq!(c, &mut [1, 2, 3]);
    assert_eq!(d, &mut [4, 5, 6]);

    println!("Everything went well !")
}

fn custom_split_at_mut<'a>(vector: &'a mut [i32], index: usize) -> (&'a mut [i32], &'a mut [i32]) {
    let length = vector.len();
    let ptr = vector.as_mut_ptr();

    assert!(length >= index);
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, index),
            slice::from_raw_parts_mut(ptr.add(index), length - index),
        )
    }
}

fn dereferencing_raw_pointers() {
    let mut x = 5;

    let p = &x as *const i32;
    let mut p1 = &mut x as *mut i32;

    unsafe {
        *p1 = 10;
        println!("Immutable pointer should have changed : {}", *p);
        println!("Mutable too : {}", *p1);
    }
}

fn calling_unsafe_functions() {
    unsafe fn dangerous() {
        println!("Calling this function is unsafe !");
    }

    unsafe {
        dangerous();
    }
}
