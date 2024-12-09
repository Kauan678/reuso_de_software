use std::io;

// Função para calcular a média dos números positivos em um array
/*fn media_positivos(arr: [i32; 10]) -> Option<f32> {
    // Inicializa as variáveis soma e qtd (quantidade) com zero
    let mut soma = 0;
    let mut qtd = 0;

    // Itera sobre os 10 elementos do array
    for i in 0..10 {
        // Se o elemento for positivo, adiciona à soma e incrementa a quantidade
        if arr[i] > 0 {
            soma += arr[i];
            qtd += 1;
        }
    }

    // Se houver pelo menos um número positivo, retorna a média como Some(valor)
    if qtd > 0 {
        Some(soma as f32 / qtd as f32)
    } else {
        // Caso contrário, retorna None
        None
    }
}
    
// Função principal para a questão 01
fn questao_01_main() {
    // Inicializa um array de 10 elementos com zeros
    let mut arr = [0; 10];

    // Lê 10 números do usuário e armazena no array
    for i in 0..10 {
        println!("Digite o {}º número:", i + 1);
        let mut num = String::new();
        io::stdin().read_line(&mut num).unwrap();
        arr[i] = num.trim().parse().unwrap();
    }

    // Calcula a média dos números positivos no array
    match media_positivos(arr) {
        // Se houver uma média válida, imprime o valor
        Some(media) => println!("Média dos positivos: {:.2}", media),
        // Caso contrário, informa que não há números positivos no vetor
        None => println!("Não há números positivos no vetor."),
    }
}*/

// Função para analisar uma tupla de três inteiros
fn analisar_tupla() {
    // Inicializa a tupla com três valores zero
    let mut tupla_inicial = (0, 0, 0);

    // Lê o primeiro número do usuário e armazena na primeira posição da tupla
    println!("Digite o primeiro número:");
    tupla_inicial.0 = ler_inteiro();
    
    // Lê o segundo número do usuário e armazena na segunda posição da tupla
    println!("Digite o segundo número:");
    tupla_inicial.1 = ler_inteiro();
    
    // Lê o terceiro número do usuário e armazena na terceira posição da tupla
    println!("Digite o terceiro número:");
    tupla_inicial.2 = ler_inteiro();
    
    // Exibe a tupla inicial
    println!("Tupla inicial: {:?}", tupla_inicial);

    // Calcula a soma dos três números da tupla
    let soma = tupla_inicial.0 + tupla_inicial.1 + tupla_inicial.2;

    // Determina o maior valor na tupla
    let maior;
    if tupla_inicial.0 > tupla_inicial.1 && tupla_inicial.0 > tupla_inicial.2 {
        maior = tupla_inicial.0;
    } else if tupla_inicial.1 > tupla_inicial.0 && tupla_inicial.1 > tupla_inicial.2 {
        maior = tupla_inicial.1;
    } else {
        maior = tupla_inicial.2;
    }

    // Determina o menor valor na tupla
    let menor;
    if tupla_inicial.0 < tupla_inicial.1 && tupla_inicial.0 < tupla_inicial.2 {
        menor = tupla_inicial.0;
    } else if tupla_inicial.1 < tupla_inicial.0 && tupla_inicial.1 < tupla_inicial.2 {
        menor = tupla_inicial.1;
    } else {
        menor = tupla_inicial.2;
    }

    // Cria uma nova tupla contendo a soma, o maior e o menor valor
    let tupla_analisada = (soma, maior, menor);
    
    // Exibe a tupla analisada
    println!("Tupla analisada: {:?}", tupla_analisada);
}

// Função para ler um inteiro da entrada padrão
fn ler_inteiro() -> i32 {
    let mut num = String::new();
    // Lê uma linha de entrada do usuário e armazena na variável `num`
    io::stdin().read_line(&mut num).expect("Falha ao ler a entrada.");
    // Converte a string lida para um inteiro e retorna
    num.trim().parse().expect("Entrada inválida.")
}

// Função principal para a questão 02
fn questao_02_main() {
    // Chama a função para analisar a tupla
    analisar_tupla();
}

// Função principal geral
fn main() {
    // Chame a função da questão que você deseja executar
    // questao_01_main();
    questao_02_main();
}