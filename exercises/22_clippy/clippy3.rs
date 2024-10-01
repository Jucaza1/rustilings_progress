// Here are some more easy Clippy fixes so you can see its utility 📎
// TODO: Fix all the Clippy lints.

use std::mem::swap;

#[rustfmt::skip]
#[allow(unused_variables, unused_assignments)]
fn main() -> Result<(),String>{
    let my_option: Option<()> = None;
    if my_option.is_some() {
        println!("{:?}", my_option.ok_or("option is none")?);
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6,
    ];
    println!("My array! Here it is: {my_arr:?}");

    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear(); // <<== clear the vector, dont use resize for 0 size, thats a unit
    // let my_empty_vec: [i32;0] = []; // <<== intialize an empty vector of i32
    println!("This Vec is empty, see? {my_empty_vec:?}");

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    // value_a = value_b;
    // value_b = value_a;
    swap(&mut value_b, &mut value_a);
    println!("value a: {value_a}; value b: {value_b}");
    Ok(())
}
