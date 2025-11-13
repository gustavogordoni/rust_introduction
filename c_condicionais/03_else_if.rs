fn main() {
    let a = 10;
    let b = 20;
    let c = 15;

    if a > b {
        println!("'a' é maior que 'b'");
    } else if b > c {
        println!("'b' é maior que 'a'");
    } else {
        //caso não aconteça nenhum dos casos cai aqui
        println!("'c' é maior que 'b'");
    }
}
