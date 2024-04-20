pub mod db_item {
use serde::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize, Clone)]
    pub struct DbItem {
        pub name: String,
        pub r#type: String,
        pub rarity: i32,
    }

    impl DbItem{ 
        pub fn new_empty_db_item() -> DbItem{
            DbItem{
                name: String::from(""),
                r#type: String::from(""),
                rarity: 0,
            }
        }

        pub fn new_db_item(name: &str, r#type: &str, rarity: i32) -> DbItem{
            DbItem{
                name: String::from(name),
                r#type: String::from(r#type),
                rarity: rarity,
            }
        }
    }

}
