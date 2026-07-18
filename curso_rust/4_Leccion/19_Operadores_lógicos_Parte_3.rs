fn main(){

    /*
    Al comprobar múltiples condiciones, el ordenador deja de verificar tan pronto como conoce la respuesta final 
    (esto se conoce como evaluación de cortocircuito).
     */

    // Ejemplo
    let x = 0;
    let y = 5;

    let resultado = x != 0 && y / x > 2;
    println!("resultado = {}",resultado);
    // Aqui x es igual a 0 por lo tanto no evaluara y / x > 2;
    
    // Si invertimos el orden 
    
    //let resultado2 = y / x > 2 && x != 0;

    // Esto resultará en un error porque y será dividido por 0, lo cual es ilegal en matemáticas.

    // Esta técnica se utiliza para optimizar la evaluación de expresiones lógicas. Por ejemplo:

    let a = 0;
    let b = 2;
    let c = 3;
    let d = 5;

    let resultado3 = (a > 0 && b < 2) || (c < -5 && d < 10);
    println!("resultado3 = {}",resultado3);
    // En este ejemplo b < 2 y d < 10 no se evaluarán porque a > 0 y c < -5 son ambos falsos.

}