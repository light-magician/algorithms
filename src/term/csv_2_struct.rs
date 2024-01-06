// read pokemon.csv and convert it to a struct
fn main() {

}

fn csv_2_struct(path: &str, ) {
    let mut rdr = csv::Reader::from_path("resources/pokemon.csv").unwrap();
    for result in rdr.deserialize() {
        let record: Record = result.unwrap();
        println!("{:?}", record);
    }
}

struct Stats {
    // values are 0-255
    hp: u8,
    attack: u8,
    defense: u8,
    sp_attack: u8,
    sp_defense: u8,
    speed: u8,
}

struct P {
    name: String,
    species: String,
    type_1: String,
    type_2: String,
    stats: Stats
}
