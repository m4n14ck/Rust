fn main(){

    /*
    En Rust, las variables son inmutables por defecto. 
    Esto significa que una vez que asignas un valor a una variable, 
    no puedes cambiar ese valor. Sin embargo, puedes hacer que una variable sea mutable 
    usando la palabra clave mut al declarar la variable.
     */

     // Por ejemplo:

     let _x = 5; // x es inmutable 
     let mut y = 10; // y es mutable 

     /*
     En este ejemplo, x es inmutable, por lo que no puedes cambiar su valor después de que se inicializa. 
     Por otro lado, y es mutable, por lo que puedes cambiar su valor más adelante en el código.
    */

    y = 20; // Esto esta permitido por que y es mutable
    println!("y = {}",y);

    // x = 15 Esto causara un error por que x es inmutable
}