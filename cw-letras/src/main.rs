use crossterm::{
    cursor, event::{read, Event, KeyCode, KeyEvent}, execute, style, terminal::{self, ClearType}
};
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::io::stdout;

fn main() {
    // Inicializa la terminal
    let mut stdout = stdout();
    terminal::enable_raw_mode().unwrap();
    execute!(stdout, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();

    let morse_table = morse_code_table();
    let mut rng = rand::thread_rng();
    let letters: Vec<char> = morse_table.keys().cloned().collect();

    loop {
        // Escoge una letra al azar
        let letter = *letters.choose(&mut rng).unwrap();
        let correct_translation = morse_table[&letter];

        // Muestra la letra y pide la traducción
        execute!(
            stdout,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0),
            style::Print(format!(
                "Traduce el siguiente caracter al código Morse: {}\n",
                letter
            )),
            cursor::MoveTo(0, 1)
        ).unwrap();

        let mut user_input = String::new();

        // Leer entrada del usuario
        loop {
            if let Event::Key(KeyEvent { code, .. }) = read().unwrap() {
                match code {
                    KeyCode::Char(c) => {
                        user_input.push(c);
                        execute!(stdout, style::Print(c)).unwrap();
                    }
                    KeyCode::Backspace => {
                        if !user_input.is_empty() {
                            user_input.pop();
                            execute!(stdout, cursor::MoveLeft(1), style::Print(" "), cursor::MoveLeft(1)).unwrap();
                        }
                    }
                    KeyCode::Esc => return,
                    KeyCode::Enter => break,
                    _ => {}
                }
            }
        }

        // Comprobar la traducción
        if user_input.trim() == correct_translation {
            execute!(
                stdout,
                cursor::MoveTo(0, 0),
                style::Print("\nCorrecto! Presiona cualquier tecla para continuar...")
            ).unwrap();
        } else {
            execute!(
                stdout,
                cursor::MoveTo(0, 0),
                style::Print(format!(
                    "\nIncorrecto. La respuesta correcta era: {}\r\nPresiona cualquier tecla para continuar...",
                    correct_translation
                ))
            ).unwrap();
        }

        // Esperar entrada para continuar
        read().unwrap();
    }
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
    table
}
