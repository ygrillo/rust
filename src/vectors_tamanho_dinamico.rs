// vetores são arrays de tamanho dinâmico
pub fn vetores() {
    // let mut notas: Vec<f32> = Vec::new();
    let mut notas: Vec<f32> = vec![10.0, 8.0, 7.5];
    // notas.push(10.0);
    // notas.push(8.0);
    // notas.push(7.5);

    println!("{:?}", notas);

    notas.push(6.3);

    println!("{:?}", notas);

    println!("Nota 6 = {}", match notas.get(3) {
        Some(&n) => n,
        None => 0.0
    });

    // println!("Nota 1 = {}", notas[0]);
    for i in 0..notas.len() {
        println!("{}", notas[i])
    }
}