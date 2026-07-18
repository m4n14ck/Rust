fn main(){

    /*
    if nos permite ejecutar código específico si se cumple una condición, 
    pero ¿qué pasa si queremos ejecutar algo más si no se cumple la condición?
    Para eso tenemos la sentencia else:
     */

     let age = 15;
     let status = if age >= 18{
        "Adult"
     } else {
        "Young"
     };
     println!("{}",status);

     // En el ejemplo anterior, age es menor que 18, 
     // lo que significa que entra en el código else y status contendrá "Young".

     // Incluso podemos hacerlo más profundo utilizando la sentencia else if:
     let age2 = 68;
     let status2 = if age2 < 18 {
        "Young"
     } else if age2 >= 18 && age2 <= 65 {
        "Adult"
     } else {
        "Old"
     };

     println!("{}",status2);
}