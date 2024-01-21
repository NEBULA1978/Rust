fn main() {
    println!("¡Hola, mundo!");

    // Declarar una variable mutable
    let mut x = 5;

    // Imprimir el valor de la variable
    println!("El valor de x es: {}", x);

    // Reasignar el valor de la variable
    x = 10;

    println!("Ahora, el valor de x es: {}", x);

    // Tipos de datos
    let _y: f32 = 3.14; // Ignorar la advertencia utilizando un guion bajo

    let number = 7;

    if number % 2 == 0 {
        println!("El número es par");
    } else {
        println!("El número es impar");
    }

    // Bucle for
    for i in 0..5 {
        println!("Iteración {}", i);
    }

    // Bucle while
    let mut j = 0;
    while j < 3 {
        println!("Bucle while: {}", j);
        j += 1;
    }
}
