fn main(){

    /*
    Un char es un solo carácter (Por ejemplo: 1, 6, %, b, p, ., T, etc.)
    
    El tipo String es un tipo especial que consiste en múltiples char.
    Para inicializar un valor de cadena en una variable, enciérrelo entre comillas dobles.
    */

    let s1 = "Esta es una string";

    /*Si quieres declarar explícitamente un tipo String, 
    necesitas convertir la cadena de esta manera 
    */

    let s2: String = "Esto es una string".to_string();

    println!("s1 = {}",s1);
    println!("s2 = {}",s2);

}