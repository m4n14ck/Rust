fn main() {
    // Podemos anidar sentencias if-elif-else unas dentro de otras.
    // Esto nos permite crear estructuras jerárquicas de toma de decisiones.

    // Por ejemplo:
    if age > 18 {
        if hasLicense {
            println!("You can drive");
        } else {
            println!("Get a license first");
        }
    } else {
        println!("Too young to drive");
    }
}
