fn main() {
    // vector is dynamic length, All items same type
    println!("Vector Collection (compound type)");

    // Initialization
    let vms: Vec<u8> = vec![0, 1, 3, 5]; // rule 1
    let mut vms2: Vec<u8> = Vec::new(); // rule 2
    let vms3: Vec<u8> = (0..10).collect(); // rule 3
    
    vms2.push(3); // adding items in mutable vector
    println!("Vector-1: {:?}",vms);
    println!("Vector-2: {:?}",vms2);
    println!("Vector-3: {:?}",vms3);
    
    let result = vms[3] + vms2[0]; // accessing vector items
    println!("Vector addition: {}",result);

    vms2[0] = 4; // Vector changing value
    println!("Vector-2: {:?}",vms2);
}
