use serde_json::{Value};
use std::fs::File;
use std::collections::HashSet;


// -- TODO --
// > krasse Rekursion um immer deeper zu diggen
// > alle daten aufsplitten um einzeln zu verarbeiten
// > wenn bei alle geguckten videos, mit zeit in watchsessions aufteilen

fn main() {
    
}

fn get_top_keys(v: &Value) -> HashSet<String> {
    let map = v.as_object().expect("expect so abfuck");
    map.keys().cloned().collect() // .cloned macht aus &String -> String, .collect ist goated und bildet aus igerator (map.keys()), was festes wie HashSet (HashSet wegen Return type)
}

// fn get_specific_values() -> Vec<_> {

// }
