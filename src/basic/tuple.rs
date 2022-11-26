fn change(mut tuple: (&str, u8, bool)) -> (&str, u8, bool) {
    tuple.0 = "Nazrul";
    tuple.1 = 26;
    return tuple;
}

fn simple(name: &str, age: u8){
    println!("My Name: {}", name);
    println!("My Age: {}", age);
    
}
fn main() {
    // Tuple exercise
    // fixed length: once declared, they cannot grow or shrink in size.

    println!("Tuple Type (compound type)");

    let mut tup: (&str, u8, bool) = ("Nobin", 30, true);
    // let bl: bool = true;

    simple(tup.0, tup.1);
    tup = change(tup);
    simple(tup.0, tup.1);
    simple(tup.0, tup.1);
}
// mutable reference
// fn change(tuple: &mut (&str, u8, bool)) {
//     tuple.0 = "Nazrul";
//     tuple.1 = 26;
// }

// fn simple(name: &str, age: u8){
//     println!("My Name: {}", name);
//     println!("My Age: {}", age);
    
// }
// fn main() {
//     // Tuple exercise
//     // fixed length: once declared, they cannot grow or shrink in size.

//     println!("Tuple Type (compound type)");

//     let mut tup: (&str, u8, bool) = ("Nobin", 30, true);
//     // let bl: bool = true;

//     simple(tup.0, tup.1);
//     change(&mut tup);
//     simple(tup.0, tup.1);
// }

