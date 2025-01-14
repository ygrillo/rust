pub fn conteudo_opcional_if_let() {
    let conteudo_arquivo = ler_arquivo_if_let(String::from(""));

    match &conteudo_arquivo {
        Some(valor) => println!("{}", valor),
        None => println!("Conteúdo não existe")
    };

    println!("{:?}", &conteudo_arquivo);

    if let Some(valor) = conteudo_arquivo {
        println!("Agora tenho certeza de que há o valor '{}'", valor);
    }
}

fn ler_arquivo_if_let(_caminho_arquivo: String) -> Option<String> {
    Some(String::from("Conteudo do arquivo"))
    // None
}