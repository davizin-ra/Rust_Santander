const ANO_ATUAL: u16 = 2026;

fn main() {
    let nome: &str = "Davi";
    let ano_nascimento: u16 = 2008;
    let mes_nascimento: u8 = 03;
    let mes_atual:u8 = 05;
    let dia_nascimento:u8 = 17;
    let dia_atual:u8 = 26;
    let mut idade: u16 = ANO_ATUAL - ano_nascimento;

    if mes_nascimento > mes_atual {
        idade -= 1
    } else if dia_nascimento >= dia_atual {
        idade -= 1
    }

    println!("{} tem {} anos", nome, idade);

    let number: i8 = 1;

    match number {
        1 => println!("Um"),
        2 => println!("Dois"),
        3 => println!("Três"),
        _ => print!("Outro número")
    }
}
