fn main() {
    let mut array: [u8; 10] = [0; 10];
    for i in 0..array.len() {
        array[i] = i as u8 + 10u8;
    }

    for (i, val) in array.iter().enumerate() {
        println!("Posição: {}, valor: {}", i, val);
    }
}
