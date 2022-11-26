fn main() {
    // Loop, Iteretor, Reapitetion

    // while
    let mut num = 5;
    while num < 10 {
        println!("nuumber = {}", num);
        num += 1;
    }

    // loop
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

    // naming loop
    let mut num = 0;
    'counter: loop {
        println!("Brazil: {}", num);
        let mut decrease = 5;
        'checking: loop {
            println!("Arzentina: {}", decrease);
            if decrease == 4 {
                break 'checking;
            }
            if num == 2 {
                break 'counter;
            }
            decrease -= 1; //decrease = decrease - 1
        }
        num += 1; // num = num + 1
    }

    // for loop
    let vec: Vec<i8> = (0..10).collect();

    for element in vec {
        println!("{}", element);
    }

    for number in (1..4).rev() {
        println!("{}", number)
    }
    println!("LIFTOFF!!!!");
}