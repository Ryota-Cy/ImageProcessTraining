extern "C" {
    fn add(a: *mut i32, b: *mut i32, c: *mut i32);
}

fn main() {
    let a = vec![1, 2, 3, 4];
    let b = vec![5, 6, 7, 8];
    let mut c = vec![0; 4];

    unsafe {
        add(a.as_ptr() as *mut i32, b.as_ptr() as *mut i32, c.as_mut_ptr());
    }

    println!("Result: {:?}", c);
}
