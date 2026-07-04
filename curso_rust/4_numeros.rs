fn main(){

    /* 
    una variable es como una unidad de memoria a la que podemos acceder 
    escribiendo el nombre de la variable. 

    Cada variable tiene un nombre único y un valor que puede ser de diferentes tipos. 
    Rust tiene varios tipos de datos integrados que definen 
    el tipo de valor que una variable puede contener.
    */

    /*
    En Rust, los números se representan normalmente utilizando dos tipos de datos 
    principales: i32 y f64.
    i32 se utiliza para almacenar números enteros sin ningún punto decimal. Por ejemplo:
    */

    let edad: i32 = 25;
    let temperatura: i32 = -5;
    let count: i32 = 100;

    // f64 se utiliza para almacenar números con un punto decimal. Por ejemplo
    let precio: f64 = 99.99;
    let pi: f64 = 3.14159;
    let fraccion: f64 = 0.5;

    println!("${}",edad);
    println!("${}",temperatura);
    println!("${}",count);

    println!("${}",precio);
    println!("${}",pi);
    println!("${}",fraccion);

}