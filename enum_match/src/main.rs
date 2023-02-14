#[derive(Debug)]

enum Season {
Spring,
Summer,
Autumn,
Winter,
}
fn main() {
    use Season::*;
    let yearly_seasons = vec![Spring, Summer, Autumn, Winter];
    for seasons in yearly_seasons {
        print!("{:?}\n", seasons as i32);
    }
}