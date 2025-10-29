use std::fs;
use std::cmp;

// --- Pentru Problema 3 ---
use serde_derive::Deserialize;

// Structura folosita pentru P1 si P3
#[derive(Debug, Clone, Deserialize)]
struct Student {
    name: String,
    phone: String,
    age: u8,
}

// --- Problema 1: Parsare CSV ---

fn solve_p1() {
    println!("--- P1: Find youngest and oldest (CSV) ---");

    let p1_content = "Constantin,0711111111,21\n\
                      Mihai,0722222222,23\n\
                      Elena,073333333333,25\n\
                      Diana,0744444444,20";
    fs::write("p1_input.txt", p1_content).expect("P1: failed to write file");
    let content = fs::read_to_string("p1_input.txt").expect("P1: failed to read file");

    let mut students: Vec<Student> = Vec::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        
        if parts.len() == 3 {
            if let Ok(age) = parts[2].trim().parse::<u8>() {
                let student = Student {
                    name: parts[0].trim().to_string(),
                    phone: parts[1].trim().to_string(),
                    age: age,
                };
                students.push(student);
            }
        }
    }

    if students.is_empty() {
        println!("P1: No valid students found.");
        return;
    }

    let mut youngest = students[0].clone();
    let mut oldest = students[0].clone();

    for student in students.iter().skip(1) {
        if student.age < youngest.age {
            youngest = student.clone();
        }
        if student.age > oldest.age {
            oldest = student.clone();
        }
    }

    println!("P1 Youngest: {:?}", youngest);
    println!("P1 Oldest: {:?}", oldest);
    println!();
}


// --- Problema 2: Canvas ---

const CANVAS_HEIGHT: usize = 55;
const CANVAS_WIDTH: usize = 100;

struct Canvas {
    grid: [[char; CANVAS_WIDTH]; CANVAS_HEIGHT],
}

impl Canvas {
    fn new() -> Self {
        Canvas {
            grid: [[' '; CANVAS_WIDTH]; CANVAS_HEIGHT],
        }
    }

    fn set_pixel(&mut self, x: usize, y: usize, value: char) {
        if y < CANVAS_HEIGHT && x < CANVAS_WIDTH {
            self.grid[y][x] = value;
        }
    }

    fn print(&self) {
        for y in 0..CANVAS_HEIGHT {
            let mut line = String::new();
            for x in 0..CANVAS_WIDTH {
                line.push(self.grid[y][x]);
            }
            println!("{}", line);
        }
    }
}

fn solve_p2() {
    println!("--- P2: Canvas 55x100 ---");
    
    let mut my_canvas = Canvas::new();

    // Draw something
    my_canvas.set_pixel(1, 1, '+');
    my_canvas.set_pixel(1, 2, '|');
    my_canvas.set_pixel(1, 3, '+');
    my_canvas.set_pixel(2, 1, '-');
    my_canvas.set_pixel(2, 3, '-');
    my_canvas.set_pixel(3, 1, '+');
    my_canvas.set_pixel(3, 2, '|');
    my_canvas.set_pixel(3, 3, '+');

    for i in 5..20 {
        my_canvas.set_pixel(i, 5, '=');
    }

    my_canvas.print();
    println!();
}


// --- Problema 3: Parsare JSON cu Serde ---

fn solve_p3() {
    println!("--- P3: Find youngest and oldest (JSON) ---");

    let p3_content = r#"{ "name": "Constantin", "phone": "0711111111", "age": 21 }
{ "name": "Mihai", "phone": "0722222222", "age": 23 }
{ "name": "Elena", "phone": "073333333333", "age": 25 }
{ "name": "Diana", "phone": "0744444444", "age": 20 }"#;
    fs::write("p3_input.jsonl", p3_content).expect("P3: failed to write file");

    let content = fs::read_to_string("p3_input.jsonl").expect("P3: failed to read file");

    let mut students: Vec<Student> = Vec::new();

    for line in content.lines() {
        if let Ok(student) = serde_json::from_str::<Student>(line) {
            students.push(student);
        } else {
            println!("Warning: Failed to parse line: {}", line);
        }
    }

    if students.is_empty() {
        println!("P3: No valid students found.");
        return;
    }

    // Logic is identical to P1
    let mut youngest = students[0].clone();
    let mut oldest = students[0].clone();

    for student in students.iter().skip(1) {
        if student.age < youngest.age {
            youngest = student.clone();
        }
        if student.age > oldest.age {
            oldest = student.clone();
        }
    }

    println!("P3 Youngest: {:?}", youngest);
    println!("P3 Oldest: {:?}", oldest);
    println!();
}


// --- Functia principala ---
fn main() {
    solve_p1();
    solve_p2();
    solve_p3();
}

