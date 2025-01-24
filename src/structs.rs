struct ContaCorrente {
    titular: Titular,
    saldo: f64
}

impl ContaCorrente {
    fn sacar(&mut self, valor: f64) {
        self.saldo -= valor;
    }
}

struct Titular {
    nome: String,
    sobrenome: String
}

impl Titular {
    fn mostrar_nome_completo(self) -> String {
        format!("{} {}", self.nome, self.sobrenome)
    }
}

pub fn conta_corrent() {
    let titular: Titular = Titular{nome: String::from("Yuri"), sobrenome: String::from("Grillo")};

    let mut conta_yuri: ContaCorrente = ContaCorrente{
        titular, 
        saldo: 100.0
    };

    conta_yuri.sacar(50.0);

    println!("Dados da conta => Titular: {} Saldo: {}", conta_yuri.titular.mostrar_nome_completo(), conta_yuri.saldo);
}