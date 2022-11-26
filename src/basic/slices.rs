fn main() {
    // slices are pointer which points to a part or whole array or vector.
    println!("slice Collection (compound type)");

    // Initialization
    let vms: Vec<u8> = (0..10).collect(); // vector
    let som: &[u8] = &vms; // rule 1 (containing full vector)
    let som2: &[u8] = &vms[3..7]; // rule 2 (containing a part of vector)
    
    println!("Vector: {:?}",vms);
    println!("Slice-1: {:?}",som);
    println!("Slices-2: {:?}",som2);
    
}
