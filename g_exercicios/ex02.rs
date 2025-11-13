// 2 - Faça um programa que calcule a média entre quatro notas e informe se foi aprovado ou não e a média, para ser aprovado a média deve ser maior ou igual a 7.
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
