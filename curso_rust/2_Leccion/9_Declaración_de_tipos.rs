fn main(){

    /*
    Una vez que una variable se declara con un tipo determinado,
    solo puede contener valores de ese tipo. 
    
    Por ejemplo, una variable i32 solo puede contener valores enteros, 
    y una variable String solo puede contener texto.
     */

    // Por ejemplo:

    let mut age: i32 = 25; // Solo puede contener numeros enteros
    let mut name: &str = "Juan"; // Solo puede contener texto

    // Esto causaria ERROR

    // age = "Patricio"; No se puede poner texto en una variable i32
    // name = 100; No se puede poner un número en una variable String

    // Estos son válidos

    age = 26;
    name = "Pablo";

    println!("age = {}",age);
    println!("name = {}",name);
}