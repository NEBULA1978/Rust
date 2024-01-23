Para compilar archivo:
rustc main.rs

Para ejecutar archivo complidado creado:
./main


Resultado por consola:

❯ ./main

¡Hola, mundo!
El valor de x es: 5
Ahora, el valor de x es: 10
El número es impar
Iteración 0
Iteración 1
Iteración 2
Iteración 3
Iteración 4
Bucle while: 0
Bucle while: 1
Bucle while: 2

////////////////////////
////////////////////////

Preguntas pagina:
https://learn.microsoft.com/en-us/training/modules/rust-introduction/3-rust-features

1. What's a compelling advantage of working with Rust? 
￼
Rust is type-safe, memory-safe, and data race-free.
Correct! Data types in Rust are verified at compile time, pointers (references) always refer to valid memory, and thread-safety is guaranteed.

￼
Rust is optimized for bare-metal development like operating systems.
￼
Rust has a robust garbage collector and helps to manage memory efficiently.
2. How is Rust code executed? 
￼
Scripts are interpreted by Rust.
￼
Rust code must be included in a C/C++ source file.
￼
Through compilation followed by direct execution.
Correct. Rust code is compiled into an executable file, and then the executable is run.

3. What's an example of something you can't do with Cargo? 
￼
Build an existing Rust project.
￼
Update the installed Rust compiler version.
Correct. You can't update the Rust compiler version by using Cargo. Use the rustup command to update the Rust compiler version.

//////////////////

 ¿Cuál es una ventaja convincente de trabajar con Rust? 

Rust es seguro para tipos, memoria y datos sin carreras.
¡Correcto! Los tipos de datos en Rust se verifican en el momento de la compilación, los punteros (referencias) siempre hacen referencia a una memoria válida y la seguridad de los subprocesos está garantizada.


Rust está optimizado para el desarrollo básico, como los sistemas operativos.

Rust tiene un recolector de basura robusto y ayuda a administrar la memoria de manera eficiente.
2. ¿Cómo se ejecuta el código Rust? 

Los guiones son interpretados por Rust.

El código Rust debe incluirse en un archivo fuente C/C++.

Mediante compilación seguida de ejecución directa.
Correcto. El código Rust se compila en un archivo ejecutable y luego se ejecuta el ejecutable.

3. ¿Cuál es un ejemplo de algo que no puedes hacer con Cargo? 

Construya un proyecto Rust existente.

Actualice la versión instalada del compilador Rust.
Correcto. No puedes actualizar la versión del compilador Rust usando Cargo. Utilice el comando rustup para actualizar la versión del compilador de Rust.

//////////////////////
//////////////////////

Comprueba tus conocimientos
Responda las siguientes preguntas para ver lo que ha aprendido. Elija una respuesta para cada pregunta y luego seleccione Compruebe sus respuestas.

￼
1. ¿Qué herramienta de juego de Rust puedes usar para encontrar errores en tu código? 
￼
oxidado
￼
clippy
Correct. You can use the Clippy collection of lint tests to find mistakes in your code and see suggestions for how to improve your code.

￼
Depurar
2. ¿Cuándo no está disponible una conexión de red en el patio de juegos de Rust? 
￼
Al editar código.
￼
Al ejecutar un programa.
￼
Al compilar código o ejecutar un programa.
Correct. When you compile code or run a program in the playground, a network connection isn't available.

////////////////////////////
////////////////////////////
Comprobación de conocimientos
Responda a las preguntas siguientes para ver lo que ha aprendido. Elija una respuesta para cada pregunta y, después, seleccione Comprobar respuestas.

￼
1. ¿Cuál es el comando recomendado que se debe usar para instalar Rust? 
￼
rinstall
￼
rustup
Correcto. El comando rustup es un instalador de cadena de herramientas para Rust. Puede actualizar el entorno a la versión estable más reciente de Rust si ejecuta el comando rustup update.

￼
rupdate
2. ¿Con qué frecuencia se actualizan las bibliotecas de Rust? 
￼
Cada seis meses
￼
Cada tres meses
￼
Cada seis semanas
Correcto. Rust tiene un proceso de versiones rápidas de seis semanas. Hay muchas compilaciones de Rust disponibles en cualquier momento.

///////////////////////
///////////////////////

Compilación y ejecución del programa con Cargo
Para ejecutar el programa reutilizable, pasaremos al nuevo directorio hello-cargo y, a continuación, usaremos el comando cargo run.

Ejecute los siguientes comandos en el terminal:

Consola
￼
Copiar
cd hello-cargo
cargo run

Debería aparecer la salida siguiente en el terminal:

Output
￼
Copiar
  Compiling hello-cargo v0.1.0 (/tmp/.OFUp/hello-cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 1.59s
      Running `target/debug/hello-cargo`

Hello, world!
Cargo ha compilado y ejecutado el archivo ejecutable.

Enhorabuena, ha escrito el primer programa de Rust y ha aprendido a inicializar el primer proyecto de Rust con Cargo.

///////////////////////////////////////
///////////////////////////////////////

Comprobación de conocimientos
Responda a las preguntas siguientes para ver lo que ha aprendido. Elija una respuesta para cada pregunta y, después, seleccione Comprobar las respuestas.


1. ¿Cuántas funciones principales puede haber en un programa de Rust? 

Un programa de Rust puede tener tantas funciones main como sea necesario.

Cualquier función de Rust puede tener una subfunción denominada main.

Cada programa de Rust solo debe tener una función llamada main.
Correcto. Un programa de Rust solo puede tener una función main.

2. ¿Qué palabra clave de Rust se usa para declarar una función nueva? 

function

fn
Correcto. La palabra clave fn se usa para declarar cualquier función, incluida main.


func
3. ¿Cuál es la salida de esta llamada a la macro println!?
println!("{} is a number. {} is a word.", 1, "Two"); 

1 is a number. Two is a word.
Correcto. La macro println! reemplaza cada instancia de llaves {} por el siguiente valor de argumento.


{} is a number. {} is a word.

{1} is a number. {"Two"} is a word.

///////////////////////////////////////////
///////////////////////////////////////////

VARIABLES

// Declare a variable
let a_number;
    
// Declare a second variable and bind the value
let a_word = "Ten";
    
// Bind a value to the first variable
a_number = 10;

println!("The number is {}.", a_number);
println!("The word is {}.", a_word);
Nuestro ejemplo imprime la siguiente salida:

Output
￼
Copiar
The number is 10.
The word is Ten.

///////////////////////////////////////
///////////////////////////////////////

Para mutar un valor, debemos usar en primer lugar la palabra clave mut para convertir en mutable el enlace de una variable.

// The `mut` keyword lets the variable be changed
let mut a_number = 10; 
println!("The number is {}.", a_number);

// Change the value of an immutable variable
a_number = 15;
println!("Now the number is {}.", a_number);

Este ejemplo imprime la salida siguiente:
The number is 10.
Now the number is 15.

////////////////////////////////////
////////////////////////////////////

Propiedad reemplazada de variables
Puede declarar una variable nueva que use el nombre de una existente. La declaración nueva crea un enlace. En Rust, esta operación se denomina "propiedad reemplazada" porque la nueva variable prevalece sobre la anterior. La antigua variable sigue existiendo, pero ya no se puede hacer referencia a ella en este ámbito.

En el código siguiente se muestra cómo usar la propiedad reemplazada. Declaramos una variable denominada shadow_num. No definimos la variable como mutable porque cada operación let crea una variable denominada shadow_num mientras se reemplaza la propiedad del enlace de la variable anterior.

// Declare first variable binding with name "shadow_num"
let shadow_num = 5;

// Declare second variable binding, shadows existing variable "shadow_num" 
let shadow_num = shadow_num + 5; 

// Declare third variable binding, shadows second binding of variable "shadow_num"
let shadow_num = shadow_num * 2; 

println!("The number is {}.", shadow_num);

Comprobación de conocimientos
Responda a las preguntas siguientes para ver lo que ha aprendido. Elija una respuesta para cada pregunta y, después, seleccione Comprobar las respuestas.

￼
1. ¿Qué instrucción de Rust declara una variable y enlaza un valor? 
￼
let continents = 7;
Correcto. La palabra clave let declara la variable y el signo igual = enlaza el valor.

￼
continents = 7;
￼
let continents;
2. ¿Qué palabra clave de Rust se usa para hacer que el valor de una variable sea modificable? 
￼
mutable
￼
immutable
￼
mut
Correcto. La palabra clave mut se usa para declarar el valor de una variable para que sea modificable.

/////////////////////////////////////////////
/////////////////////////////////////////////

Exploración de tipos de datos para números, texto y valores true/false
200 XP
7 minutos
Rust es un lenguaje con establecimiento de tipos en modo estático. El compilador debe conocer el tipo de datos exacto de todas las variables del código para que el programa se compile y ejecute. Normalmente, el compilador puede inferir el tipo de datos de una variable en función del valor enlazado. No siempre es necesario indicar de forma explícita el tipo en el código. Cuando son posibles muchos tipos, debe informar al compilador del tipo específico mediante anotaciones de tipo.

En el ejemplo siguiente, se le indica al compilador que cree la variable number como un entero de 32 bits. Especificamos el tipo de datos u32 después del nombre de la variable. Observe que después del nombre de la variable se usa el carácter de dos puntos :.

Rust

Copiar
let number: u32 = 14;
println!("The number is {}.", number);
Si ponemos el valor de la variable entre comillas dobles, el compilador interpreta el valor como texto en lugar de como un número. El tipo de datos deducido del valor no coincide con el tipo de datos u32 especificado para la variable, por lo que el compilador emite un error:

Rust

Copiar
let number: u32 = "14";
Error del compilador:

Output

Copiar
   Compiling playground v0.0.1 (/playground)
error[E0308]: mismatched types
 --> src/main.rs:2:23
  |
2 |     let number: u32 = "14";
  |                 ---   ^^^^ expected `u32`, found `&str`
  |                 |
  |                 expected due to this

error: aborting due to previous error
Se puede interactuar con el código anterior en esta área de juegos de Rust.

Tipos de datos integrados
Rust incluye algunos tipos de datos primitivos integrados para expresar números, texto y veracidad. Algunos de estos tipos se conocen como escalares, porque representan un solo valor:

Números enteros
Números de punto flotante
Valores booleanos
Characters
Rust también ofrece tipos de datos más complejos para trabajar con series de datos, como valores de cadena y de tupla.

Números: valores enteros y de punto flotante
Los enteros en Rust se identifican por el tamaño en bits y la propiedad signed. Un entero con signo puede ser un número positivo o negativo. Un entero sin signo solo puede ser un número positivo.

Length	Firmado	Sin signo	 	 
8 bits	i8	u8	 	 
16 bits	i16	u16	 	 
32 bits	i32	u32	 	 
64 bits	i64	u64	 	 
128 bits	i128	u128	 	 
dependiente de la arquitectura	isize	usize	 	 
Los tipos isize y usize dependen del tipo de equipo en el que se ejecuta el programa. El tipo de 64 bits se usa en una arquitectura de 64 bits y el tipo de 32 bits, en una arquitectura de 32 bits. Si no especifica el tipo para un entero, y el sistema no puede deducir el tipo, asigna el tipo i32 (un entero de 32 bits con signo) de forma predeterminada.

Rust tiene dos tipos de datos de punto flotante para los valores decimales: f32 (32 bits) y f64 (64 bits). El tipo de punto flotante predeterminado es f64. En las CPU modernas, el tipo f64 tiene aproximadamente la misma velocidad que el tipo f32, pero cuenta con una mayor precisión.

Rust

Copiar
let number_64 = 4.0;      // compiler infers the value to use the default type f64
let number_32: f32 = 5.0; // type f32 specified via annotation
Todos los tipos de números primitivos en Rust admiten operaciones matemáticas como suma, resta, multiplicación y división.

Rust

Copiar
// Addition, Subtraction, and Multiplication
println!("1 + 2 = {} and 8 - 5 = {} and 15 * 3 = {}", 1u32 + 2, 8i32 - 5, 15 * 3);

// Integer and Floating point division
println!("9 / 2 = {} but 9.0 / 2.0 = {}", 9u32 / 2, 9.0 / 2.0);
 Nota

Cuando llamamos a la macro println, agregamos el sufijo de tipo de datos a cada número literal para informar a Rust sobre el tipo de datos. La sintaxis 1u32 indica al compilador que el valor es el número 1 y que interprete el valor como un entero de 32 bits sin signo.

Si no se proporcionan anotaciones de tipo, Rust intenta deducir el tipo a partir del contexto. Cuando el contexto es ambiguo, asigna el tipo i32 (un entero de 32 bits con signo) de forma predeterminada.

Se puede intentar ejecutar este ejemplo en el área de juegos de Rust.

Valores booleanos: true o false
El tipo booleano en Rust se usa para almacenar la veracidad. El tipo bool tiene dos valores posibles: true o false. Los valores booleanos se usan de forma generalizada en expresiones condicionales. Si una instrucción bool o un valor es true, realice esta acción; de lo contrario (la instrucción o el valor es false), realice una acción distinta. Una comprobación de comparación suele devolver un valor booleano.

En el ejemplo siguiente, usamos el operador mayor que > para probar dos valores. El operador devuelve un valor booleano que muestra el resultado de la prueba.

Rust

Copiar
// Declare variable to store result of "greater than" test, Is 1 > 4? -- false
let is_bigger = 1 > 4;
println!("Is 1 > 4? {}", is_bigger);  
Texto: caracteres y cadenas
Rust admite valores de texto con dos tipos de cadena básicos y un tipo de carácter. Un carácter es un elemento único, mientras que una cadena es una serie de caracteres. Todos los tipos de texto son representaciones UTF-8 válidas.

Characters
El tipo char es el más primitivo de los tipos de texto. El valor se especifica poniendo el elemento entre comillas simples:

Rust

Copiar
let uppercase_s = 'S';
let lowercase_f = 'f';
let smiley_face = '😃';
 Nota

Algunos lenguajes tratan sus tipos char como enteros de 8 bits sin signo, que es el equivalente del tipo u8 de Rust. El tipo char de Rust contiene puntos de código Unicode, pero no usa la codificación UTF-8. char en Rust es un entero de 21 bits que se ha agregado para ampliar a 32 bits. char contiene directamente el valor de punto de código sin formato. Puede obtener más información sobre el tipochar de Rust en la documentación de Rust.

Cadenas
El tipo str, también conocido como segmento de cadena, es una vista de los datos de la cadena. La mayoría de las veces, se hace referencia a estos tipos usando la sintaxis del estilo de referencia que precede al tipo con el símbolo de y comercial &str. Trataremos las referencias en los siguientes módulos. Por ahora, puede imaginarse &str como un puntero a datos de cadena inmutables. Los literales de cadena son todos de tipo &str.

Aunque los literales de cadena son convenientes para usarlos en ejemplos de introducción de Rust, no son adecuados para todas las situaciones en las que podríamos querer usar texto. No todas las cadenas pueden conocerse en tiempo de compilación. Un ejemplo se da cuando un usuario interactúa con un programa en tiempo de ejecución y envía texto mediante un terminal.

En estos escenarios, Rust tiene un segundo tipo de cadena denominado String. Este tipo se asigna en el montón. Cuando se usa el tipo String, no es necesario conocer la longitud de la cadena (número de caracteres) antes de compilar el código.

 Nota

Si está familiarizado con un lenguaje de recolección de elementos no utilizados, es posible que se pregunte por qué Rust tiene dos tipos de cadena. 1 Las cadenas son tipos de datos extremadamente complejos. La mayoría de los lenguajes usan sus recolectores de elementos no utilizados para atenuar esta complejidad. Rust, como lenguaje de un sistema, expone parte de la complejidad inherente de las cadenas. La complejidad adicional conlleva una cantidad de control muy específica sobre cómo se usa la memoria en el programa.

1 _En realidad, Rust tiene más de dos tipos de cadena. En este módulo, solo se describen los tipos String y &str. Puede obtener más información sobre los tipos de cadena que se ofrecen en la documentación de Rust.

No va a obtener una idea completa de la diferencia entre String y &str hasta que conozca la propiedad y el sistema de préstamos de Rust. Hasta entonces, puede pensar en los datos de tipo String como datos de texto que pueden cambiar a medida que se ejecuta el programa. Las referencias &str son vistas inmutables en los datos de texto que no cambian a medida que se ejecuta el programa.

Ejemplo de texto
En el ejemplo siguiente se muestra cómo usar los tipos de datos char y &str en Rust.

Se declaran dos variables de caracteres con la sintaxis de anotación : char. Los valores se especifican usando comillas simples.
Se declara una tercera variable de caracteres y se enlaza a una sola imagen. Para esta variable, se permite que el compilador deduzca el tipo de datos.
Se declaran dos variables de cadena y se enlazan a sus valores respectivos. Las cadenas se ponen entre comillas dobles.
Una de las variables de cadena se declara con la sintaxis de anotación : &str para especificar el tipo de datos. El tipo de datos de la otra variable se deja sin especificar. El compilador deducirá el tipo de datos de esta variable en función del contexto.
Observe que la variable string_1 incluye un espacio vacío al final de la serie de caracteres.

Rust

Copiar
// Specify the data type "char"
let character_1: char = 'S';
let character_2: char = 'f';
   
// Compiler interprets a single item in quotations as the "char" data type
let smiley_face = '😃';

// Compiler interprets a series of items in quotations as a "str" data type and creates a "&str" reference
let string_1 = "miley ";

// Specify the data type "str" with the reference syntax "&str"
let string_2: &str = "ace";

println!("{} is a {}{}{}{}.", smiley_face, character_1, string_1, character_2, string_2);
Esta es la salida de nuestro ejemplo:

Output

Copiar
😃 is a Smiley face.
¿Qué ocurre si no se especifica el símbolo de y comercial & antes de str en este ejemplo? Para averiguarlo, intente ejecutar este ejemplo en el Área de juegos de Rust.

Comprobación de conocimientos
Responda a las preguntas siguientes para ver lo que ha aprendido. Elija una respuesta para cada pregunta y, después, seleccione Comprobar las respuestas.


1. ¿Qué afirmación describe cómo se definen los valores numéricos enteros en Rust? 

En Rust, los enteros se identifican principalmente por su tamaño de bits, por ejemplo 8 bits, 16 bits, y así sucesivamente.

En Rust, los enteros se identifican por su tamaño en bits y la propiedad con signo.

En Rust, un entero positivo o negativo se puede definir como un valor sin signo (u) o con signo (i).
2. ¿Qué afirmación describe correctamente cómo se admiten en Rust los valores de caracteres de texto? 

Rust tiene un tipo de datos que se puede usar tanto para caracteres únicos como para cadenas de texto de varios caracteres.

Un carácter (char) solo puede ser una sola letra alfa, como "A" o "z". Una cadena puede ser una serie de caracteres cualquiera: letras, números, imágenes, entre otros.

En Rust, todos los tipos de texto son representaciones UTF-8 válidas.


/////////////////////////////////

let uppercase_s = 'S';
let lowercase_f = 'f';
let smiley_face = '😃';

/////////////////////////////////

Comprobación de conocimientos
Responda a las preguntas siguientes para ver lo que ha aprendido. Elija una respuesta para cada pregunta y, después, seleccione Comprobar las respuestas.

￼
1. ¿Qué afirmación describe cómo se definen los valores numéricos enteros en Rust? 
￼
En Rust, los enteros se identifican principalmente por su tamaño de bits, por ejemplo 8 bits, 16 bits, y así sucesivamente.
￼
En Rust, los enteros se identifican por su tamaño en bits y la propiedad con signo.
Correcto. Un entero se identifica mediante una combinación de su tamaño de bits y si tiene signo (i) o no tiene signo (u).

￼
En Rust, un entero positivo o negativo se puede definir como un valor sin signo (u) o con signo (i).
2. ¿Qué afirmación describe correctamente cómo se admiten en Rust los valores de caracteres de texto? 
￼
Rust tiene un tipo de datos que se puede usar tanto para caracteres únicos como para cadenas de texto de varios caracteres.
￼
Un carácter (char) solo puede ser una sola letra alfa, como "A" o "z". Una cadena puede ser una serie de caracteres cualquiera: letras, números, imágenes, entre otros.
￼
En Rust, todos los tipos de texto son representaciones UTF-8 válidas.
Correcto. En Rust, todos los tipos de texto son representaciones UTF-8 válidas.

///////////////////////
///////////////////////