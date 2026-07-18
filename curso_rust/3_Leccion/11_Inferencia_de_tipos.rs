fn main(){

    /*
    En Rust, la inferencia de tipos permite al compilador deducir automáticamente 
    el tipo de una variable basándose en su valor y uso. 
    
    Esto significa que a menudo no es necesario especificar explícitamente el tipo al 
    declarar una variable, haciendo que su código sea más conciso y fácil de leer.
     */


     // Por ejemplo:

     let x = 5; // Infiere que x es un i32

     let y = 3.14; // Rust infiere que y es un f64

     let message = "Hola mundo"; // Rust infiere que message es un &str(cadena)

     let is_true = true; // Rust infiere que es un tipo

     println!("x = {}",x);
     println!("y = {}",y);
     println!("message = {}",message);
     println!("is_true = {}",is_true);

}