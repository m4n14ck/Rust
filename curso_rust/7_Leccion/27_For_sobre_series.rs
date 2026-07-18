fn main() {
    /*
    A veces, al programar, es necesario realizar la misma operación o casi la misma operación un par de veces.
    Para evitar escribir lo mismo una y otra vez, podemos usar bucles.
     */

    /*
     Una expresión range define cuántas veces debe ejecutarse el bucle, escrita normalmente como start..end
     (que se ejecuta desde start hasta end-1) o start..=end (que se ejecuta desde start hasta end, incluyendo end).
    Por ejemplo, un bucle de 0 a 5 (sin incluir):
      */

    for i in 0..=5 {
        println!("{}", i);
    }

    // Los bucles tienen muchos casos de uso. Por ejemplo, sumemos todos los números del 1 al 100:

    let mut sum_nunmbers = 0;

    for i in 1..=100 {
        sum_nunmbers += i;
    }

    println!("{}", sum_nunmbers);

    // Esto primero recorrerá todos los números entre 1 y 100 (incluyendo el 100 debido al signo ..=)
    // y los sumará todos, luego imprimirá la variable sum_numbers

    // Si por alguna razón quieres crear un bucle sin usar una variable (i),
    // deberías añadir un guion bajo al principio del nombre: _i.
    // Esto le dirá al compilador que está bien que no se use, y evitará que el programa genere una advertencia:

    for _i in 0..5 {
        println!("Hello!");
    }
}
