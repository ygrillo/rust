mod escopos;
mod funcoes;
mod condicionais;
mod repeticoes;
mod ownership;
mod pattern_matching;
mod tratamento_erros;
mod agrupar_dados;
mod criacao_tipos;


fn main() {
    escopos::scope();
    escopos::shadow();
    funcoes::fn_sum(1, 2);
    condicionais::condicionais();
    repeticoes::repeticoes();
    ownership::ownership();
    pattern_matching::pattern_matching();
    tratamento_erros::erros();
    agrupar_dados::calcula_nota();
    agrupar_dados::matriz();
    criacao_tipos::valida_fim_de_semana(criacao_tipos::DiaDaSemana::Domingo);
}
