pub fn condicionais() {
    let idade: u8 = 17;
    let responsavel_autorizou: bool = true;
    let maior_idade: bool = idade >= 18;
    if maior_idade {
        println!("Pode entrar na balada.");
    } else if idade > 16 && responsavel_autorizou {
        println!("Pode entrar na balada com assinatura do responsável.");
    } else {
        println!("Não pode entrar na balada!");
    }

    let condicao: &str = if idade >= 18 { "maior" } else { "menor" };
    println!("É {} de idade.", condicao);

    let linguagem = "Python";
    let proposito = match linguagem {
        "PHP" => "Web",
        "Kotlin" => "Android",
        "Python" => "Data Science",
        _ => "Desconhecido"
    };

    println!("O propósito de {} é {}", linguagem, proposito);

}
