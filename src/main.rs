use std::env;
mod lib;

fn main() {
    
    let args: Vec<String> = env::args().collect();
    let query = &args[1];

    

    if query.to_ascii_lowercase().eq("newhistory")  {

        let history_filename = get_history_filename();
        
        let history_dump = lib::lib_history::get_history(history_filename);

        for line in history_dump {
            println!("{}",line)
        }
    }

}

fn get_history_filename() -> String {

    let  user_name = match  env::var("HOME") {
         Ok(home) => home,
         Err(err) =>  { 
             panic!("Cannot find history file {}", err);
         }
    };

    let history_file = String::from("/.bash_history");

    let default_file = user_name + &history_file;

    println!("Path is {}", &default_file);

    default_file
}