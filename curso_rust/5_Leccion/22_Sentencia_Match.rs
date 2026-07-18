fn main(){

    // La expresión match permite comparar un valor con una serie de patrones y 
    // ejecutar código en función del patrón que coincida.

    // Aquí está la estructura básica de una expresión match:

    /*
    match variable {
        pattern1 => expression1,
        pattern2 => expression2,
        // ... más patrones
        _ => default_expression,
    }
     */

     /*
     La palabra clave match va seguida del valor que desea probar.
     Cada brazo de la coincidencia consiste en un patrón seguido de => y el código a ejecutar.
     El guion bajo _ es el caso por defecto que coincide con cualquier cosa que no haya sido emparejada por otros patrones.
      */

      let day = 3;
      let day_name = match day {
        1 => "Lunes",
        2 => "Martes",
        3 => "Miercoles",
        4 => "Jueves",
        5 => "Viernes",
        6 => "Sabado",
        7 => "Domingo",
        _=> "Dia invalido",
      };

      println!("{}",day_name);

      // Puedes hacer coincidir múltiples patrones usando |:

      let day = 3;
      let day_type = match day {
        1 | 2 | 3 | 4 | 5 => "Weekday",
        6 | 7 => "Weekend",
        _ => "Invalid day",
    };

}