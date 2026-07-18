fn main(){

    /*
    Los operadores lógicos tienen una tabla especial llamada "Tabla de verdad" 
    que muestra lo que devuelve la combinación de operadores lógicos.

    Tabla de verdad para el operador and (&&):

      a	      b	    a && b
    false	false	false
    false	true	false
    true	false	false
    true	true	true

    La única forma de obtener un true para el operador and (&&) es si tanto a como b son true

    Tabla de verdad para el operador or (||):

      a	      b     a || b
    false	false	false
    false	true	true
    true	false	true
    true	true	true
    En este caso, para obtener un resultado true, ya sea a o b debe ser true.

    Tabla de verdad para el operador not (!):

     a	     !a
    false	true
    true	false
    Aquí el valor de a se invierte. Si a es false entonces !a es true
     */

    // Escribe tu código debajo
    let b1 = 15;
    let b2 = 10;
    let b3 = !((b1 + b2) > (b1 * b2));
    
    // No cambies la línea de abajo
    println!("b3 = {}", b3);
}