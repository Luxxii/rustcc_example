
extern { fn test(a: i32) ->i32 ; }

fn main() {
    println!("Hello, world!");

    unsafe { test(3); } 
}

