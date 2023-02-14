fn main() {
    let months = vec!["January", "February", "March", "April", "May", 
    "June", "July", "August", "September", "October", "November", "December"];

    let filtered_months = months
        // into_iter() metho call is an owned iterator. Mmonths no longer exists
        .into_iter() // makes owned iterator
        .filter(|month| month.len() < 5) // Each letter is 1 byte.
        // therefore we do not want months of lenghth greater than 4 bytes or letters
        .filter(|month| month.contains("u"))// filter months which
        // contain the letter u 
        .collect::<Vec<&str>>(); // create new vector for the filtered_months
        // remmeber iterators are lazy so you need to csonume them and use them hence why we use collect

        println!("{:?}", filtered_months);
        // code below will not work because months value has been moved to 
        // new binding filtered_months
        // println!("{:?}", months);
}