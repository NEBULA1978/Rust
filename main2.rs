fn main() {
   // Declaración de un número entero de 32 bits con anotación de tipo
let number: u32 = 14;
println!("El número es {}.", number);

// Intento incorrecto de asignar una cadena a una variable u32
// Esto generará un error de compilación
// let number: u32 = "14";

// Declaración de números de punto flotante con inferencia de tipo
let number_64 = 4.0;      // el compilador infiere el tipo f64 por defecto
let number_32: f32 = 5.0; // especificamos el tipo f32 mediante anotación

// Operaciones matemáticas con diferentes tipos de números
println!("1 + 2 = {} y 8 - 5 = {} y 15 * 3 = {}", 1u32 + 2, 8i32 - 5, 15 * 3);
println!("9 / 2 = {} pero 9.0 / 2.0 = {}", 9u32 / 2, 9.0 / 2.0);

// Declaración de valores booleanos y uso en expresiones condicionales
let is_bigger = 1 > 4;
println!("¿Es 1 > 4? {}", is_bigger);

// Declaración de caracteres y cadenas
let character_1: char = 'S';
let character_2: char = 'f';
let smiley_face = '😃';

let string_1 = "miley ";
let string_2: &str = "ace";

// Concatenación e impresión de caracteres y cadenas
println!("{} es una {}{}{}{}.", smiley_face, character_1, string_1, character_2, string_2);

}
