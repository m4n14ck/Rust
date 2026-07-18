use std::io;

fn main() {
    /*
    ==========================
    Entrada de datos en Rust
    ==========================

    Hasta ahora hemos creado programas donde los valores están escritos
    directamente en el código.

    Sin embargo, la mayoría de los programas necesitan interactuar con el
    usuario y recibir información desde el teclado.

    Para ello, Rust utiliza la biblioteca estándar `std::io`.
    */

    // Importar la biblioteca
    // use std::io;

    /*
    -----------------------------------
    Leer una cadena (String)
    -----------------------------------

    Toda la información que escribe el usuario se recibe inicialmente
    como una cadena (String).
    */

    let mut my_var = String::new();
    // Crea una cadena vacía donde se almacenará la entrada.

    io::stdin().read_line(&mut my_var).unwrap();
    // Lee lo que escribe el usuario hasta que presiona Enter.

    println!("El usuario escribió: {}", my_var);

    /*
    IMPORTANTE

    Si el usuario escribe:

        Hola

    En realidad Rust guarda:

        "Hola\n"

    Ese '\n' corresponde al Enter que presionó el usuario.

    Para eliminar ese salto de línea utilizamos:

        trim()

    */

    println!("{}", my_var.trim());

    /*
    -----------------------------------
    Convertir la entrada a otro tipo
    -----------------------------------

    Como toda la entrada llega como String, si queremos trabajar con
    números debemos convertir el texto utilizando `parse()`.

    Sintaxis general:

        let variable: Tipo = texto.trim().parse().unwrap();

    o también

        let variable = texto.trim().parse::<Tipo>().unwrap();

    Ambos hacen exactamente lo mismo.
    */

    /*
    ===================================
    Ejemplos de conversión
    ===================================
    */

    // Entero con signo de 32 bits
    let entero: i32 = my_var.trim().parse().unwrap();

    // Entero sin signo
    let entero_positivo: u32 = my_var.trim().parse().unwrap();

    // Entero grande
    let entero_grande: i64 = my_var.trim().parse().unwrap();

    // Decimal de precisión simple
    let decimal: f32 = my_var.trim().parse().unwrap();

    // Decimal de doble precisión
    let decimal_grande: f64 = my_var.trim().parse().unwrap();

    // Booleano
    // El usuario debe escribir exactamente:
    // true
    // false
    let booleano: bool = my_var.trim().parse().unwrap();

    // Carácter
    // El usuario debe escribir un solo carácter.
    let caracter: char = my_var.trim().parse().unwrap();

    /*
    ===================================
    ¿Por qué debemos indicar el tipo?
    ===================================

    parse() puede convertir un String a muchos tipos diferentes.

    Por ejemplo:

        i32
        u32
        i64
        f32
        f64
        bool
        char

    Si escribimos simplemente:

        let numero = my_var.trim().parse().unwrap();

    Rust mostrará un error porque no sabe a qué tipo queremos convertir
    el texto.

    Por eso debemos especificarlo:

        let numero: i32 = my_var.trim().parse().unwrap();

    o

        let numero = my_var.trim().parse::<i32>().unwrap();

    */

    /*
    ===================================
    Resumen
    ===================================

    1. Crear un String vacío.

        let mut entrada = String::new();

    2. Leer la entrada.

        io::stdin().read_line(&mut entrada).unwrap();

    3. Si solo necesitas texto:

        println!("{}", entrada.trim());

    4. Si necesitas otro tipo de dato:

        let numero: i32 = entrada.trim().parse().unwrap();
    */
}