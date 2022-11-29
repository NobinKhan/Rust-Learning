struct User {
    name: String,
    age: u8,
    married: bool,
    is_adult: bool,
}

impl User {
    fn check(&self) -> bool {
        if self.is_adult && self.age > 18{
            return true;
        }
        else if self.married || self.age < 18{
            return false;
        }
        else {
            false
        }
    }
}

fn main() {
    // struct
    let nobin = User{
        name: "Nobin".to_string(),
        age: 9,
        married: false,
        is_adult: true
    };
    println!("{}", nobin.name);
    println!("check = {}", nobin.check());

}