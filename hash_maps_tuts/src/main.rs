// CREATING A HASHMAP
// HASMAP NOT INCLUDED INPRELUDE SO HAVE TO EXPLICITYLY IMPORT IT FROM 
// THE STANDARD LIBRARY USING a use statement from collections.
// Hashmap is a homogenous collection data set or container that is 
// expandable and is stored on the heap instead of the stack because 
// it is resizable

// CREATE HASHMAP WITH NEW AND UPDATE HASHMAP WITH INSERT

// HashMap import use statement
use std::collections::HashMap;

// pub fn with_capacity(capacity: usize) -> HashMap<K, V, RandomState> {
//     HashMap::with_capacity_and_hasher(capacity, Default::default())
// }

// let mut club_one = fn with_capacity(11);
// let mut club_two = fn with_capacity(11);

// CREATING HASHMAP
fn main () {
    // Instantiating clubs using HashMap
    let mut club_one = HashMap::new();
    let mut club_two = HashMap::new();



    // Updating respective clubs by adding new members
    club_one.insert(String::from("Alice"), 10);
    club_two.insert(String::from("Bob"), 50);
    club_one.insert(String::from("Jesus"), 1);

    for (key, value) in &(club_one) {
        println!("{}, {}", key, value);
    }
    for (key, value) in &(club_two) {
        println!("{}, {}", key, value);
    }
     println!("Club one: {:?} and club two: {:?}", club_one, club_two);
}