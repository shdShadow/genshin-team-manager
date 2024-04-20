pub mod db_reader {
    use crate::entities::db_item::db_item::DbItem;
    use std::{env, io::Error};
    pub fn read_db() -> Result<Vec<DbItem>, Error> {
        if let Ok(current_dir) = env::current_dir() {
            println!("Current directory: {:?}", current_dir);
        } else {
            println!("Unable to get the current directory.");
        }
        let db_content =
            std::fs::read_to_string("./resources.json").expect("Error reading db.json");
        let vec: Vec<DbItem> = serde_json::from_str(&db_content).expect("Error parsing db.json");
        Ok(vec)
    }
}
