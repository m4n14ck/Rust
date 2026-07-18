fn main(){

    // Las expresiones if nos permiten ejecutar código con condiciones.

    // Por ejemplo, veamos el siguiente código:

    let mut age = 20;
    let mut status = "child";

    if age > 18 {
        status = "Adult";
    }
    age += 1;

    println!("age = {}",age);
    println!("status = {}",status);

    // El código anterior comprueba si la variable age es mayor que 18. 
    // Si lo es, establecerá status para que contenga la cadena "Adult".

    // Al final, el código incrementará age en 1 independientemente de si la edad es mayor que 18 o no.
    // Para usar una sentencia if en Rust, necesitamos usar llaves {} 
    // para definir el bloque de código, y todo lo que esté dentro de la sentencia if debe colocarse entre estas llaves:

    /*
        if condition {
            code;
            code;
            code;
        }
     */
    
}