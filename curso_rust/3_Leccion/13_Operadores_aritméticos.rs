fn main(){

    /*
    Los operadores se utilizan para realizar operaciones sobre los valores.
    Primero discutiremos los operadores aritméticos más básicos, 
    pueden resultarte familiares de las clases de matemáticas.

    Operador	Operación	    Ejemplo
        +	      Suma	        3 + 2 = 5
        -	      Resta	        3 - 2 = 1
        *	  Multiplicación	3 * 2 = 6
        /	      División	    4 / 2 = 2
     */

     // Veamos un ejemplo de uso,

     let a = 32;
     let b = 5;
     let c = a + b; // c contiene 8

     println!("c = {}",c);

     /*
     Al trabajar con números decimales en Rust, utilizamos el tipo de dato f64, 
     que puede almacenar números con puntos decimales. 

     Los mismos operadores aritméticos (+, -, *, /) 
     funcionan con f64 al igual que lo hacen con los enteros
      */

    let x = 3.3;
    let y = 4.1;
    let z = x + y;

    println!("z = {}",z);  
}