mod agrupar_dados;

const PI: f32 = 3.14;
static VARIAVEL_GLOBAL: u8 = 1;

fn fn_sum(a:i32, b: i32) -> i32 {
    println!("{} + {} = {}", a, b, a + b);
    a + b
}

fn shadow() {
    let a: u32 = 123;

    {
        let b: u32 = 456;
        println!("dentro, b = {}", b);
        let a: u32 = 777;  // Variável completamente nova. Não é redeclaração.
        println!("dentro, a = {}", a);
    }
    // println!("fora, b = {}", b);
    println!("fora, a = {}", a);
}

fn scope() {
    println!("PI = {}", &PI);
    println!("Variável Global = {}", &VARIAVEL_GLOBAL);

    let variable:i32 = 128;
    println!("Variável = {}, tamanho = {} bytes", &variable, std::mem::size_of_val(&variable));

    let decimal_value: f32 = 2.5;
    println!("Decimal = {}", &decimal_value);

    let booleana: bool = false;
    println!("Tamanho booleana = {}, valor = {}", std::mem::size_of_val(&booleana), &booleana);

    let letra: char = 'C';
    println!("Caracter = {}, Tamanho do char = {}", &letra, std::mem::size_of_val(&letra));
}

fn main() {
    scope();
    shadow();
    fn_sum(1, 2);
    condicionais();
    repeticoes();
    ownership();
    pattern_matching();
    erros();
    agrupar_dados::calcula_nota();
    agrupar_dados::matriz();
}

fn condicionais() {
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

fn repeticoes() {
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

fn ownership() {
    let mut uma_string = String::from("Yuri");
    rouba(&mut uma_string);
    println!("{}", uma_string);
}

fn rouba(string: &mut String) {
    string.push_str(" Grillo");
    println!("{}", string);
}

fn pattern_matching() {
    for x in 1..=20 {
        println!("{}: {}", x, match x {
            1 => "Pouco",
            2 | 3 => "Um pouquinho",
            4..=10 => "Um bocado",
            _ if x % 2 == 0 => "Uma boa quantidade",
            _ => "Muito"
        })
    }
}

fn erros() {
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
