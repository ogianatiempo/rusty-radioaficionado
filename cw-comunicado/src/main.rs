use std::collections::HashMap;
use std::io;
use rand::seq::SliceRandom; // Importa `choose`
use rand::thread_rng;       // Generador de números aleatorios

fn main() {
    let comunicados = vec![
        vec![
            "CQ CQ DE LW9DVR LW9DVR K",
            "LW9DVR DE LU8DMA KN",
            "LU8DMA DE LW9DVR RST",
            "599 DNN DNN QTH LOMAS KN",
            "R R LW9DVR DE LU8DMA",
        ],
        vec![
            "CQ CQ DE LU1XYZ LU1XYZ K",
            "LU1XYZ DE LU5ABC RST 599 QSL?",
            "QSL R LU5ABC DE LU1XYZ TU 73",
            "QRZ? DE LU2JKL",
            "LU2JKL DE LW8OPR KN",
        ],
        vec![
            "QRV? DE LU3DEF",
            "LU3DEF DE LW7GHI QRS PSE",
            "QRS R LU3DEF DE LW7GHI TU",
            "CQ DX DE LU4JKL",
            "LU4JKL DE LU6MNO PSE K",
        ],
    ];

    // Selección aleatoria
    let comunicado = comunicados.choose(&mut thread_rng()).unwrap();

    println!("Comunicado:");
    for line in comunicado {
        println!("{}", line);
    }

    // Tabla de código Morse
    let morse_table = morse_code_table();

    println!("\nTraduce el comunicado a código Morse línea por línea.");
    println!("Usa rayas (-) y puntos (.) para cada línea.");

    for (line_number, original_line) in comunicado.iter().enumerate() {
        println!("\nTraduce la línea {}:", line_number + 1);
        println!("{}", original_line);

        let morse_translation = original_line
            .to_uppercase()
            .chars()
            .filter_map(|c| morse_table.get(&c).cloned())
            .collect::<Vec<_>>()
            .join(" ");

        // Entrada del usuario
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Error al leer la entrada");
        let user_input = user_input.trim();

        // Comparar traducción
        if user_input == morse_translation {
            println!("¡Correcto!");
        } else {
            println!("Traducción incorrecta.");
            println!("Tu traducción:\n{}", user_input);
            println!("{}", highlight_errors(user_input, &morse_translation));
            println!("La traducción correcta es:\n{}", morse_translation);
        }
    }

    println!("\n¡Has completado la traducción del comunicado!");
}

fn highlight_errors(user_input: &str, correct_translation: &str) -> String {
    let mut error_marks = String::new();
    for (user_char, correct_char) in user_input.chars().zip(correct_translation.chars()) {
        if user_char != correct_char {
            error_marks.push('^');
        } else {
            error_marks.push(' ');
        }
    }

    // Agregar marcas para caracteres faltantes o extra
    let length_difference = correct_translation.len() as isize - user_input.len() as isize;
    if length_difference > 0 {
        error_marks.push_str(&"^".repeat(length_difference as usize));
    }

    error_marks
}

fn morse_code_table() -> HashMap<char, &'static str> {
    let mut table = HashMap::new();
    table.insert('A', ".-");
    table.insert('B', "-...");
    table.insert('C', "-.-.");
    table.insert('D', "-..");
    table.insert('E', ".");
    table.insert('F', "..-.");
    table.insert('G', "--.");
    table.insert('H', "....");
    table.insert('I', "..");
    table.insert('J', ".---");
    table.insert('K', "-.-");
    table.insert('L', ".-..");
    table.insert('M', "--");
    table.insert('N', "-.");
    table.insert('O', "---");
    table.insert('P', ".--.");
    table.insert('Q', "--.-");
    table.insert('R', ".-.");
    table.insert('S', "...");
    table.insert('T', "-");
    table.insert('U', "..-");
    table.insert('V', "...-");
    table.insert('W', ".--");
    table.insert('X', "-..-");
    table.insert('Y', "-.--");
    table.insert('Z', "--..");
    table.insert('1', ".----");
    table.insert('2', "..---");
    table.insert('3', "...--");
    table.insert('4', "....-");
    table.insert('5', ".....");
    table.insert('6', "-....");
    table.insert('7', "--...");
    table.insert('8', "---..");
    table.insert('9', "----.");
    table.insert('0', "-----");
    table.insert(' ', "/"); // Separador de palabras
    table.insert('.', ".-.-.-"); // Punto
    table.insert(',', "--..--"); // Coma
    table.insert('?', "..--.."); // Signo de pregunta
    table.insert('!', "-.-.--"); // Signo de exclamación
    table.insert('-', "-....-"); // Guion
    table.insert('/', "-..-.");  // Barra
    table.insert('(', "-.--.");  // Paréntesis abierto
    table.insert(')', "-.--.-"); // Paréntesis cerrado
    table.insert('@', ".--.-."); // Arroba
    table.insert('=', "-...-");  // Signo igual
    table
}
