use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use rusqlite::{Connection, Result as SqlResult, params};

// --- Definirea Trait-ului pentru Comenzi ---
trait Command {
    fn get_name(&self) -> &'static str;
    fn exec(&mut self, args: &[String]);
}

// --- Comenzile pentru P1 ---

struct PingCommand;
impl Command for PingCommand {
    fn get_name(&self) -> &'static str { "ping" }
    fn exec(&mut self, _: &[String]) { println!("pong!"); }
}

struct CountCommand;
impl Command for CountCommand {
    fn get_name(&self) -> &'static str { "count" }
    fn exec(&mut self, args: &[String]) { println!("counted {} args", args.len()); }
}

struct TimesCommand { count: u32 }
impl Command for TimesCommand {
    fn get_name(&self) -> &'static str { "times" }
    fn exec(&mut self, _: &[String]) {
        self.count += 1;
        println!("command called {} times", self.count);
    }
}

// Comanda custom ceruta de P1
struct HelloCommand;
impl Command for HelloCommand {
    fn get_name(&self) -> &'static str { "hello" }
    fn exec(&mut self, args: &[String]) {
        if args.is_empty() {
            println!("Hello, world!");
        } else {
            println!("Hello, {}!", args.join(" "));
        }
    }
}

// --- Comanda Bookmark pentru Bonus (P2) ---

struct BookmarkCommand {
    conn: Connection,
}

impl BookmarkCommand {
    fn new() -> SqlResult<Self> {
        let conn = Connection::open("bookmarks.db")?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS bookmarks (
                name TEXT NOT NULL,
                url  TEXT NOT NULL
            )",
            (),
        )?;
        Ok(BookmarkCommand { conn })
    }

    fn add(&self, name: &str, url: &str) {
        match self.conn.execute("INSERT INTO bookmarks (name, url) VALUES (?1, ?2)", params![name, url]) {
            Ok(_) => println!("Bookmark added successfully."),
            Err(e) => println!("Error adding bookmark: {}", e),
        }
    }

    fn search(&self, query: &str) {
        let mut stmt = match self.conn.prepare("SELECT name, url FROM bookmarks WHERE name LIKE ?1") {
            Ok(s) => s,
            Err(e) => { println!("Error preparing search query: {}", e); return; }
        };

        let search_pattern = format!("%{}%", query);
        let bookmark_iter = match stmt.query_map(params![search_pattern], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        }) {
            Ok(iter) => iter,
            Err(e) => { println!("Error executing search query: {}", e); return; }
        };

        println!("Search results for '{}':", query);
        let mut found = false;
        for bookmark in bookmark_iter {
            if let Ok((name, url)) = bookmark {
                println!("  {} -> {}", name, url);
                found = true;
            }
        }
        if !found {
            println!("  No bookmarks found.");
        }
    }
}

impl Command for BookmarkCommand {
    fn get_name(&self) -> &'static str { "bk" }

    fn exec(&mut self, args: &[String]) {
        if args.is_empty() {
            println!("Usage: bk add <name> <url> OR bk search <name>");
            return;
        }

        match args[0].as_str() {
            "add" => {
                if args.len() == 3 {
                    self.add(&args[1], &args[2]);
                } else {
                    println!("Usage: bk add <name> <url>");
                }
            }
            "search" => {
                if args.len() == 2 {
                    self.search(&args[1]);
                } else {
                    println!("Usage: bk search <name>");
                }
            }
            _ => println!("Unknown bk subcommand. Use 'add' or 'search'."),
        }
    }
}

// --- Structura Terminal ---

struct Terminal {
    commands: Vec<Box<dyn Command>>,
}

impl Terminal {
    fn new() -> Self {
        Terminal { commands: Vec::new() }
    }

    fn register(&mut self, command: Box<dyn Command>) {
        self.commands.push(command);
    }

    fn run(&mut self) {
        let file = match File::open("commands.txt") {
            Ok(f) => f,
            Err(_) => {
                println!("Could not open commands.txt. Reading from stdin instead (type 'stop' to quit):");
                self.run_interactive();
                return;
            }
        };

        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line_content) = line {
                self.process_line(&line_content);
                if line_content.trim() == "stop" { return; }
            }
        }
    }

    fn run_interactive(&mut self) {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        let mut buffer = String::new();

        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            buffer.clear();
            match handle.read_line(&mut buffer) {
                Ok(0) => break, // EOF
                Ok(_) => {
                    let trimmed = buffer.trim();
                    if trimmed == "stop" { break; }
                    self.process_line(trimmed);
                }
                Err(e) => { println!("Error reading line: {}", e); break; }
            }
        }
    }

    fn process_line(&mut self, line: &str) {
        let parts: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        if parts.is_empty() { return; }

        let cmd_name = &parts[0];
        let args = &parts[1..];

        if cmd_name == "stop" { return; }

        let mut found = false;
        for cmd in &mut self.commands {
            if cmd.get_name() == cmd_name {
                cmd.exec(args);
                found = true;
                break;
            }
        }

        if !found {
            print!("Unknown command: '{}'.", cmd_name);
             // Simple suggestion logic (case-insensitive check)
             for cmd in &self.commands {
                if cmd.get_name().eq_ignore_ascii_case(cmd_name) {
                    print!(" Did you mean '{}'?", cmd.get_name());
                    break;
                }
            }
            println!();
        }
    }
}

fn main() {
    let mut terminal = Terminal::new();

    terminal.register(Box::new(PingCommand));
    terminal.register(Box::new(CountCommand));
    terminal.register(Box::new(TimesCommand { count: 0 }));
    terminal.register(Box::new(HelloCommand));

    // Register Bookmark command only if DB connection succeeds
    match BookmarkCommand::new() {
        Ok(bk_cmd) => terminal.register(Box::new(bk_cmd)),
        Err(e) => println!("Failed to initialize BookmarkCommand (SQLite error): {}", e),
    }

    terminal.run();
}