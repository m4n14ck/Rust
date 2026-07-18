
fn main(){

    // La sentencia break detiene el bucle instantáneamente cuando se encuentra.

    // Por ejemplo,

    for i in 0..10 {
        if i == 6 {
            break;
        }

        println!("{}",i);
    }

    // En el siguiente ejemplo, el bucle itera regularmente hasta que llega al número 6. 
    // Luego, el programa entra en la sentencia if y ejecuta la sentencia break. Esto sale del bucle inmediatamente.

    // La salida es:
    /*
        0
        1
        2
        3
        4
        5
     */

}