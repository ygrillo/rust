pub fn valida_fim_de_semana(dia_da_semana: DiaDaSemana) {
    println!("É fim de semana? R: {}", eh_fim_de_semana(dia_da_semana));
    cores();
}

fn eh_fim_de_semana(dia_da_semana: DiaDaSemana) -> bool {
    match dia_da_semana {
        DiaDaSemana::Domingo | DiaDaSemana::Sábado => true,
        _ => false
    }
}

#[allow(dead_code)]
pub enum DiaDaSemana {
    Domingo,
    Segunda,
    Terça,
    Quarta,
    Quinta,
    Sexta,
    Sábado
}

#[allow(dead_code)]
pub enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8),
    CymkColor{ cyan: u8, magenta: u8, yellow: u8, black: u8 }
}

fn cores() {
    let cor = Color::CymkColor { cyan: 100, magenta: 50, yellow: 70, black: 255 };

    println!("{}", match cor {
        Color::Red => "vermelho",
        Color::Green => "verde",
        Color::Blue => "azul",
        Color::RgbColor(0, 0, 0) | Color::CymkColor { cyan: _, magenta: _, yellow: _, black: 255 } => "preta",
        Color::RgbColor(_, _, _) => "RGB desconhecido",
        Color::CymkColor { cyan: _, magenta: _, yellow: _, black: _ } => "CYMK desconhecido"
    });
}