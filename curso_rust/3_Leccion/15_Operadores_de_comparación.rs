fn main(){

    /*
    Los Operadores de comparación se utilizan para comparar entre dos operandos.
    A veces necesitamos comprobar si un operando es mayor/menor/... que otro operando. 
    La siguiente tabla muestra los posibles operadores de comparación:

     Operador	    Significado	        Ejemplo
        ==	            Igual	    1 == 2 devuelve false
        !=	        No es igual	    1 != 2 devuelve true
        >	        Mayor que	    1 > 2 devuelve false
        <	        Menor que	    1 < 2 devuelve true
        >=	        Mayor o igual	1 >= 2 devuelve false
        <=	        Menor o igual	1 <= 2 devuelve true
     */

    // El operador de comparación devuelve true si la comparación es correcta o false de lo contrario.

    let var1 = 13;
    let var2 = 12;
    let var3 = var1 != var2; // var3 contendrá true porque var1 y var2 no son iguales

    println!("var3 = {}",var3);

}