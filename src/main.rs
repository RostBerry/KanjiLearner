use std::{collections::HashMap, fmt::Display};
use std::io;
use serde::{Deserialize, Serialize};
use serde_json::{self, Result};
use std::fs;

#[derive(Serialize, Deserialize)] 
struct KanjiInfo {
    ids: Vec<u16>,
    occasions: u16
}

impl KanjiInfo {
    fn new(id: u16) -> Self {
        KanjiInfo{ids: vec![id], occasions: 1}
    }
}

struct KanjiRepresentation {
    page: u16,
    row: u16
}

impl KanjiRepresentation {
    fn new(page: u16, row: u16) -> Self {
        KanjiRepresentation{page, row}
    }
}

impl Display for KanjiRepresentation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "page #{}, line #{}", self.page, self.row)
    }
}

#[derive(Serialize, Deserialize)]
struct Database {
    items: HashMap<char, KanjiInfo>,
    current_id: u16,
    kanji_per_row: u16,
    rows_per_page: u16
}

impl Database {
    const FILENAME: &'static str = "data.json";

    fn new(kanji_per_row: u16, rows_per_page: u16) -> Self {
        Database {
            items: HashMap::new(),
            current_id: 0,
            kanji_per_row,
            rows_per_page
        }
    }

    fn add_kanji(&mut self, kanji: char) {
        match self.items.get_mut(&kanji) {
            Some(info) => {
                let id = *info.ids.last().expect("Error");
                println!("Found kanji {} with id #{} ({})", kanji, id, Self::get_kanji_line(id, self.rows_per_page));
                info.occasions += 1;
                println!("Writing occasion #{} for the kanji {}", info.occasions, kanji);
                if Self::have_space_in_line_ended(info.occasions, self.kanji_per_row) {
                    println!("No space left for the id #{}, creating new id #{} ({})", id, self.current_id, Self::get_kanji_line(self.current_id, self.rows_per_page));
                    info.ids.push(self.current_id);
                    self.current_id += 1;
                }
            },

            None => {
                println!("Kanji {} not found", kanji);
                println!("Putting kanji {} on the id #{} ({})", kanji, self.current_id, Self::get_kanji_line(self.current_id, self.rows_per_page));
                self.items.insert(kanji, KanjiInfo::new(self.current_id));
                self.current_id += 1;
            }
        }
        
        self.save_data().expect("Critical saving error");
    }

    fn have_space_in_line_ended(occasions: u16, kanji_per_row: u16) -> bool {
        occasions % kanji_per_row == 1
    }

    fn get_kanji_line(id: u16, rows_per_page: u16) -> KanjiRepresentation { //returns user-friendly representation of where the kanji is in the notebook (page, row)
        let page = id / rows_per_page + 1;
        let row = id % rows_per_page + 1;
        KanjiRepresentation::new(page, row)
    }

    fn save_data(&mut self) -> Result<()> {
        let json = serde_json::to_string_pretty(&self)?;
        fs::write(Self::FILENAME, json).expect("Failed to save data");
        Ok(())
    }

    fn load_data() -> Result<Self> {
        if let Ok(json) = fs::read_to_string(Self::FILENAME) {
            let db: Database = serde_json::from_str(&json).expect("Reading error");
            println!("Database loaded successfully");
            return Ok(db)
        } else {
            println!("Failed to load data, creating new database");
            println!("Enter number of kanji that can fit in one line of your notebook: ");
            let mut answer_kanji = String::new();
            let kanji_per_row: u16;
            io::stdin().read_line(&mut answer_kanji).expect("Error");
            match answer_kanji.trim().parse::<u16>() {
                Ok(number) => {
                    kanji_per_row = number;
                },
                Err(_) => {
                    panic!("Critical error");
                }      
            }
            
            println!("Enter number of rows that can fit in one page of your notebook: ");
            let mut answer_rows = String::new();
            let rows_per_page: u16;
            io::stdin().read_line(&mut answer_rows).expect("Error");
            match answer_rows.trim().parse::<u16>() {
                Ok(number) => {
                    rows_per_page = number;
                },
                Err(_) => {
                    panic!("Critical error");
                }      
            }
            return Ok(Database::new(kanji_per_row, rows_per_page));
        }
        
    }
}

fn main() {

    let mut db = Database::load_data().expect("Critical error");
    loop {
        println!("Enter kanji: ");
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("Error");
        let kanji = answer.trim();
        if kanji.chars().count() != 1 {
            println!("Invalid input");
            continue;
        }

        db.add_kanji(kanji.chars().next().unwrap());
    }
}