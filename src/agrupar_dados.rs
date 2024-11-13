pub fn calcula_nota() {
    let notas: [f32; 5] = [6.5; 5]; // inicializa todas as notas de valor 6.5
    let inteiro: usize = 0;

    println!("{}", notas[inteiro]);

    for indice in 0..notas.len() {
        println!("A nota {} é = {}", indice + 1, notas[indice]);
    }
}

pub fn matriz() {
    let matriz: [[f32; 3]; 2] = [
            [0.0, 1.2, 0.1],
            [1.3, 0.3, 1.4]
    ];

    for linha in matriz {
        for item in linha {
            println!("Item = {}", item);
        }
    }
}
