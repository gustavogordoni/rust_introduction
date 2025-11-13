fn main() {
    let mut array: [u8; 10] = [0; 10];
    for i in 0..array.len() {
        array[i] = i as u8 + 10u8;
    }

    for val in array {
        println!("Valor: {}", val);
    }
}
