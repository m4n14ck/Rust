fn main(){

    /*
    La conversión de tipos (type casting) es el proceso de convertir un valor de un tipo de datos a otro. 
    En Rust, utilizamos la palabra clave as para la conversión de tipos explícita 
    (también conocida como conversión de tipos). 
     */

     // conversion de entero a flotante

     let numero: i32 = 5;
     let decimal: f64 = numero as f64; // se convierte en 5.0

     // Conversion de punto flotante a entero
     let decimal2: f64 = 9.7;
     let numero2: i32 = decimal2 as i32; // se convierte en 9(la parte decimal se trunca)

     println!("Conversion de entero a flotante");
     println!("Numero = {}",numero);
     println!("Conversion a flotante = {}",decimal);

     println!("\nConversion de punto flotante a entero");
     println!("decimal2 = {}",decimal2);
     println!("Conversion a entero = {}",numero2);

}