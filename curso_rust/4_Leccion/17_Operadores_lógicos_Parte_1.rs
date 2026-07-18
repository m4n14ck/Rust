fn main(){

    /*
    Los operadores lógicos se utilizan para comprobar combinaciones de comparaciones que devuelven true o false.

    Por ejemplo, la siguiente declaración contiene dos comparaciones: 

    ¿Es 5 mayor que 3 y menor que 6?

    Operador	        Significado	                                Ejemplo
    &&	        And - true si todos los operandos son true	        a && b
    ||	        Or - true si cualquier operando es true	            a || b
    !	        Not - true si el operando es false	                  !a
     */

    // 5 es mayor que 3 y 1 es igual a 1,
    let b1 = (5 > 3) && (1 == 1);
    println!("b1 = {}",b1);

    // Explicación: Todos los operandos son true, 
    // por lo que b1 contendrá true (la operación and es true si ambos operandos son true) .

    // 5 no es igual a 4 o 5 es igual a 2,
    let b2 = (5 != 4) || (5 != 2);
    println!("b2 = {}",b2);

    // 1 no es igual a 1 o falso,
    let b3 = !(1 == 1) || false;
    println!("b3 = {}",b3);

    // no (3 es mayor que 4)
    let b4 = !(3 > 4);
    println!("b4 = {}",b4);

}