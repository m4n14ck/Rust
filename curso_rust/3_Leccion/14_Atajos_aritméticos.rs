fn main(){

    /*
    Rust creó un atajo genial para las operaciones de auto-aritmética.
    Por ejemplo, en lugar de escribir:
     */

    let mut a = 5;
    println!("a sin atajo = {}",a);
   
    a = a + 3; // a ahora contiene 8  
    println!("a = {}",a);

    // podemos simplificarlo escribiendo += 

    let mut b = 5;
    println!("\nb sin atajo = {}",b);
    
    b += 3; // b contiene 8
    println!("b = {}",b);

    /*
    Esta operación es válida para todas las operaciones aritméticas:

    Operador	Atajo
        +	     +=
        -	     -=
        *	     *=
        /	     /=
        %	     %=
     */

}