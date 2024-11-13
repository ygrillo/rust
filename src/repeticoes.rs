pub fn repeticoes() {
    let multiplicador: u8 = 5;
    let mut contador: u8 = 0;
    while contador < 10 {
        contador += 1; 
        if contador == 5 {
            continue;
        }
        println!("{} x {} = {}", multiplicador, contador, multiplicador * contador);
    }
    println!("{}", "-".repeat(50));
    contador = 0;
    loop {
        contador += 1;
        println!("{} x {} = {}", multiplicador, contador, multiplicador * contador);
        if contador == 10 {
            break;
        }
    }
    println!("{}", "-".repeat(50));

    for i in 1..=10 {
        println!("{} x {} = {}", multiplicador, contador, multiplicador * i);
    }
    
}
