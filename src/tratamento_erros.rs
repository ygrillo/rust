pub fn erros() {
    match resultado(8) {
        Ok(s) => println!("String de sucesso = {}", s),
        Err(numero) => println!("Código de erro = {}", numero)
    };
}

fn resultado(valor: u32) -> Result<String, u32> {
    if valor >= 10 {
        Err(404)
    } else {
        Ok(String::from("Deu tudo certo"))
    }
}