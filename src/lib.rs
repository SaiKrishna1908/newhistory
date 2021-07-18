pub use std::env;
pub use std::iter;

pub mod lib_history {

    pub fn get_history(file_name: String) -> Vec<String> {

        let history_file = file_name;
        let history_dump =   

        match std::fs::read_to_string(history_file) {
                Ok(f) => f,            
                Err(_e) => panic!("Cannot read history file"),
        };

        let mut history_vec: Vec<String> = Vec::new();

        for (i, line) in history_dump.split("\n").enumerate() {
            history_vec.push(i.to_string()+" "+line);
        }

        history_vec
    } 
}