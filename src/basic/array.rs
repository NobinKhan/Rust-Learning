fn main() {
    // Array is fixed length, All items same type
    println!("Array Collection (compound type)");

    // Initialization
    let tmm = [0, 1, 3, 5]; // rule 1
    let mut tmm2: [u8; 5] = [45, 12, 6, 7, 24]; // rule 2

    println!("array-1: {}",tmm[0]);
    println!("array-2: {}",tmm2[0]);
    
    tmm2[3] += 3;
    
    println!("array-2: {}",tmm2[tmm[2]]);
}
