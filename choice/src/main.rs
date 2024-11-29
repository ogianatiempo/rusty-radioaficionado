use std::fs;
use std::io::{self, Write};
use std::collections::HashSet;
use serde::Deserialize;
use rand::seq::SliceRandom;
use crossterm::{
    terminal::{Clear, ClearType}, cursor::MoveTo, ExecutableCommand,
};

enum TipoPregunta {
    Tecnica,
    Reglamento
}

#[derive(Deserialize)]
struct Pregunta {
    pregunta: String,
    opciones: Vec<String>,
    indices_correctas: HashSet<usize>,
}

struct Puntaje {
    tecnica_score: u32,
    reglamento_score: u32,
    tecnica_count: u32,
    reglamento_count: u32,
}

fn main() {
    let mut stdout = io::stdout();
    stdout.execute(Clear(ClearType::All)).unwrap();

    // Cargar las preguntas
    let contenido_tecnica = fs::read_to_string("tecnica.json").expect("No se pudo leer el archivo tecnica.json");
    let contenido_reglamento = fs::read_to_string("reglamento.json").expect("No se pudo leer el archivo reglamento.json");
    let mut preguntas_tecnica: Vec<Pregunta> = serde_json::from_str(&contenido_tecnica).expect("No se pudo parsear el archivo tecnica.json");
    let mut preguntas_reglamento: Vec<Pregunta> = serde_json::from_str(&contenido_reglamento).expect("No se pudo parsear el archivo reglamento.json");

    // Mezclamos las preguntas
    let mut rng = rand::thread_rng();
    preguntas_tecnica.shuffle(&mut rng);
    preguntas_reglamento.shuffle(&mut rng);


    // Inicializamos el puntaje
    let mut puntaje = Puntaje{
        tecnica_score: 0, 
        reglamento_score: 0, 
        tecnica_count: 0, 
        reglamento_count: 0 
    };
    
    while !preguntas_tecnica.is_empty() || !preguntas_reglamento.is_empty() {
        stdout.execute(Clear(ClearType::All)).unwrap();
        stdout.execute(MoveTo(0, 0)).unwrap();

        // Display current scores at the top
        println!("Puntaje   | Técnica: {} | Reglamento: {}", puntaje.tecnica_score, puntaje.reglamento_score);
        println!("Preguntas | Técnica: {} | Reglamento: {}", puntaje.tecnica_count, puntaje.reglamento_count);

        // Ask user which question set to use
        println!("\nElegí una opción:");
        println!("1. Pregunta de técnica");
        println!("2. Pregunta de reglamento");
        println!("3. Salir");
        print!("\nOpción: ");
        io::stdout().flush().unwrap();
        let mut opcion = String::new();
        io::stdin().read_line(&mut opcion).unwrap();

        match opcion.trim() {
            "1" => {
                puntaje.tecnica_count += 1;
                match preguntas_tecnica.pop() {
                    Some(pregunta) => realizar_pregunta(pregunta, TipoPregunta::Tecnica, &mut puntaje),
                    None => {
                        println!("No hay más preguntas de técnica");
                        std::thread::sleep(std::time::Duration::from_secs(2));
                        continue;
                    }
                }                
            },
            "2" => {
                puntaje.reglamento_count += 1;
                match preguntas_reglamento.pop() {
                    Some(pregunta) => realizar_pregunta(pregunta, TipoPregunta::Reglamento, &mut puntaje),
                    None => {
                        println!("No hay más preguntas de reglamento");
                        std::thread::sleep(std::time::Duration::from_secs(2));
                        continue;
                    }
                }
            },
            "3" => {
                break;
            },
            _ => {
                println!("Opción inválida.");
                continue;
            }
        };
    }

    // Puntaje final
    stdout.execute(Clear(ClearType::All)).unwrap();
    stdout.execute(MoveTo(0, 0)).unwrap();
    println!("Puntaje   | Técnica: {} | Reglamento: {}", puntaje.tecnica_score, puntaje.reglamento_score);
    println!("Preguntas | Técnica: {} | Reglamento: {}", puntaje.tecnica_count, puntaje.reglamento_count);
}

fn realizar_pregunta(question: Pregunta, tipo: TipoPregunta, puntaje: &mut Puntaje) {
    println!("\nPregunta: {}", question.pregunta);
    for (i, option) in question.opciones.iter().enumerate() {
        println!("{}. {}", i + 1, option);
    }
    print!("\nRespuesta: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let answer: HashSet<usize> = input
        .split(',')
        .filter_map(|s| s.trim().parse::<usize>().ok())  // Only keep valid numbers
        .map(|i| i - 1)
        .collect();
    
    if answer == question.indices_correctas {
        println!("Correcto!");
        match tipo {
            TipoPregunta::Tecnica => puntaje.tecnica_score += 1,
            TipoPregunta::Reglamento => puntaje.reglamento_score += 1
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    } else {
        if question.indices_correctas.len() == 1 {
            println!("Incorrecto! La respuesta correcta era: {}", question.indices_correctas.iter().next().unwrap() + 1);
            std::thread::sleep(std::time::Duration::from_secs(2));
        } else {
            println!("Incorrecto! La respuestas correctas eran: {:?}", question.indices_correctas.iter().map(|i| i + 1).collect::<Vec<usize>>());
            std::thread::sleep(std::time::Duration::from_secs(4));
        }
    }
}
