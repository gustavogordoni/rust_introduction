// 1 - Faça um programa que tenha uma função que recebe um array de inteiros com sinal (aceite números negativos) e devolva a soma dos valores deste array e exiba no console.
//

fn main() {
    let mut array: [i16; 10] = [0; 10];

    for i in 0..array.len() {
        array[i] = i as i16 - 5; // (-5 até 4)
    }

    // let resultado = soma_elementos_array(array);
    // println!("A soma dos elementos é: {}", resultado);

    println!("A soma dos elementos é: {}", soma_elementos_array(array));
}

fn soma_elementos_array(array: [i16; 10]) -> i16 {
    let mut soma: i16 = 0;

    for i in 0..array.len() {
        soma += array[i];
    }
    soma
}
