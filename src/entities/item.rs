pub mod item_node{
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Item{
        name: String,
        quantity: i32,
        possessed_quantity: i32,
    }

    impl Item{
        pub fn create_empty_item() -> Item{
            Item{
                name: String::from(""),
                quantity: 0,
                possessed_quantity: 0,
            }
        }

        pub fn create_item(name: &str, quantity: i32, possessed_quantity: i32) -> Item{
            Item{
                name: String::from(name),
                quantity,
                possessed_quantity,
            }
        }
    }
}