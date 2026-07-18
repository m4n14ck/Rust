use std::io;
use std::io::Write;

fn main(){
    print!("Digite un numero: ");
    io::stdout().flush().unwrap();

    let mut var1 = String::new();
    io::stdin().read_line(&mut var1).unwrap();
    let num1: f64 = var1.trim().parse().unwrap();

    print!("Digite otro numero: ");
    io::stdout().flush().unwrap();

    let mut var2 = String::new();
    io::stdin().read_line(&mut var2).unwrap();
    let num2: f64 = var2.trim().parse().unwrap();

    let suma = num1 + num2;
    let resta = num1 - num2;
    let multi = num1 * num2;
    let div = num1 / num2;

    println!("\nSuma: {:.2}",suma);
    println!("Resta: {:.2}",resta);
    println!("Multiplicacion: {:.2}",multi);
    println!("Divicion: {:.2}",div);    

}