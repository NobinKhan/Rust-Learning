fn main() {
    // Loop, Iteretor, Reapitetion

    // while loop
    let mut num = 5;
    while num < 10 {
        println!("nuumber = {}", num);
        num += 1;
    }
    
    loop {
        if num == 12 {
            println!("Ek Dozon");
        }
        else if num == 15 {
            num += 1;
            continue;
        }
        else if num == 20 {
            break;
        }
        println!("Holy sholy Mother Teresa");
        num += 1;
    }
}