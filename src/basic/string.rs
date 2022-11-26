fn main() {
    // strings type (String, &str)
    println!("String and &str<Doesn't allocate memory on the heap> (compound type)");

    // Initialization
    let name: String = String::from("Md. Nazrul Islam Khan"); // String type
    let phone: String = "+8801754730922".to_string(); // String type
    let email: &str = "nobin@gmail.com"; // &str type
    
    // Some Example use case.
    details(&name, email, &phone);
    println!("{}", email.replace("nobin", "nazrul"));
    details(&name, email, &phone);
    
    // Convert between String and &str
    let str_slice: &str = &name;
    let string_literal: String = str_slice.to_string();
    
    println!("String to &str = {}", str_slice);
    println!("&str to String = {}", string_literal);
}

fn details(name: &String, email: &str, phone: &String){
    println!("\nName: String = {}",name);
    println!("Email: &str = {}",email);
    println!("phone: String = {}\n",phone);
}