mod model;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use model::Gender;
use slab::Slab;
use std::collections::HashMap;

pub use model::QueryRoot;
pub type CustomersSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub struct CustomersChar {
    id: &'static str,
    name: &'static str,
    friends: Vec<usize>,
    appears_in: Vec<Gender>,
    home_planet: Option<&'static str>,
    primary_function: Option<&'static str>,
}

pub struct Customers {
    luke: usize,
    artoo: usize,
    chars: Slab<CustomersChar>,
    human_data: HashMap<&'static str, usize>,
}

impl Customers {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut chars = Slab::new();

        let luke = chars.insert(CustomersChar {
            id: "1000",
            name: "Luke Skywalker",
            friends: vec![],
            appears_in: vec![],
            home_planet: Some("Tatooine"),
            primary_function: None,
        });

        let vader = chars.insert(CustomersChar {
            id: "1001",
            name: "Anakin Skywalker",
            friends: vec![],
            appears_in: vec![],
            home_planet: Some("Tatooine"),
            primary_function: None,
        });

        let han = chars.insert(CustomersChar {
            id: "1002",
            name: "Han Solo",
            friends: vec![],
            appears_in: vec![Gender::Male],
            home_planet: None,
            primary_function: None,
        });

        let leia = chars.insert(CustomersChar {
            id: "1003",
            name: "Leia Organa",
            friends: vec![],
            appears_in: vec![Gender::Male],
            home_planet: Some("Alderaa"),
            primary_function: None,
        });

        let tarkin = chars.insert(CustomersChar {
            id: "1004",
            name: "Wilhuff Tarkin",
            friends: vec![],
            appears_in: vec![Gender::Female, Gender::Male],
            home_planet: None,
            primary_function: None,
        });

        let threepio = chars.insert(CustomersChar {
            id: "2000",
            name: "C-3PO",
            friends: vec![],
            appears_in: vec![Gender::Female],
            home_planet: None,
            primary_function: Some("Protocol"),
        });

        let artoo = chars.insert(CustomersChar {
            id: "2001",
            name: "R2-D2",
            friends: vec![],
            appears_in: vec![Gender::Female],
            home_planet: None,
            primary_function: Some("Astromech"),
        });

        chars[luke].friends = vec![han, leia, threepio, artoo];
        chars[vader].friends = vec![tarkin];
        chars[han].friends = vec![luke, leia, artoo];
        chars[leia].friends = vec![luke, han, threepio, artoo];
        chars[tarkin].friends = vec![vader];
        chars[threepio].friends = vec![luke, han, leia, artoo];
        chars[artoo].friends = vec![luke, han, leia];

        let mut human_data = HashMap::new();
        human_data.insert("1000", luke);
        human_data.insert("1001", vader);
        human_data.insert("1002", han);
        human_data.insert("1003", leia);
        human_data.insert("1004", tarkin);

        Self {
            luke,
            artoo,
            chars,
            human_data,
        }
    }

    pub fn human(&self, id: &str) -> Option<usize> {
        self.human_data.get(id).cloned()
    }

    pub fn humans(&self) -> Vec<usize> {
        self.human_data.values().cloned().collect()
    }

}
