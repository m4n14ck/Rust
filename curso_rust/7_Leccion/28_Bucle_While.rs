use std::io;
fn main() {
    /*
    Un bucle while es diferente del bucle for.
    Un bucle for nos permite iterar sobre un range específico, mientras que un bucle while
    nos permite seguir iterando mientras se cumpla una cierta condición.
     */

    // Para usar un bucle while escribe:

    /*
        while condition {
            code
        }
    */

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut number: f64 = input.trim().parse().unwrap();
    // Escribe tu código a continuación

    while number >= 3.5 {
        number = number / 2.0;
    }

    println!("{}", number);

    // El código se ejecutará solo si la condición es true.

    // Hay muchos casos de uso en los que un while resolvería el problema, pero el bucle for no lo haría.
}
