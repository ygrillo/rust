pub fn conteudo_opcional() {
    let conteudo_arquivo = ler_arquivo(String::from(""));

    match &conteudo_arquivo {
        Some(valor) => println!("{}", valor),
        None => println!("Conteúdo não existe")
    };

    println!("{:?}", &conteudo_arquivo)
}

fn ler_arquivo(_caminho_arquivo: String) -> Option<String> {
    Some(String::from("Conteudo do arquivo"))
    // None
}