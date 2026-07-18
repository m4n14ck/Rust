fn main(){

    
    // Hay dos formas principales de crear cadenas en Rust:
        
     // Método 1: Literales de cadena simples (str):
       
        let str1 = "hello";
        let str2 = "hello";
        let str3 = "Hello";
     
    
    // Estas son cadenas simples y fijas que están integradas en su programa.

    // Método 2: Cadenas completas (String):
    let string1: String = "hello".to_string();
    let string2: String = String::from("hello");
    let string3 = "hello".to_owned();

    /*
    Estas crean un tipo de cadena más flexible que puedes cambiar más tarde. 
    Las tres líneas hacen básicamente lo mismo, son solo formas diferentes de escribirlo.

    El :: en String::from es la sintaxis de Rust para llamar a una función asociada directamente en un tipo; 
    piénsalo como la propia forma integrada de String para crear un nuevo valor de cadena.
     */
}