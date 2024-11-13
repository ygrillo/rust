const PI: f32 = 3.14;
static VARIAVEL_GLOBAL: u8 = 1;

pub fn scope() {
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

pub fn shadow() {
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