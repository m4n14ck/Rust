fn main() {
    /*
    En Rust, puedes imprimir la salida en la consola utilizando las macros print! y println!.
    Estas macros te permiten mostrar texto, variables y expresiones en la consola.
    La principal diferencia entre print! y println! es que println! añade un carácter de nueva línea al final de la salida,
    lo que provoca que la siguiente salida comience en una línea nueva, mientras que print! no lo hace.
     */

    // Aquí tienes cómo puedes usar estas macros:

    let name = "Alice";
    let age = 30;

    print!("Name: ");
    print!("{}", name);
    println!(" is {} years old.", age);

    println!("Hello, {}!", name);
}
