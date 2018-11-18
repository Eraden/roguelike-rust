extern crate clap;
extern crate rand;

pub mod layer_generator;
pub mod pond;

use clap::{App, Arg};
use std::fs::write;

use layer_generator::*;

// cargo run -p map_generator -- first_map --width 40 --height 40 --layers animals ground1 ground2 ground3 players roofs plants
fn main() {
    let possible_layers = [
        "ground1", "ground2", "ground3", "animals", "plants", "players", "roofs",
    ];
    let matches = App::new("Map Generator")
        .version("1.0.0")
        .about("Simple generate map")
        .arg(
            Arg::with_name("layers")
                .short("l")
                .long("layers")
                .help("number of map layers")
                .takes_value(true)
                .possible_values(&possible_layers)
                .empty_values(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("width")
                .required(true)
                .short("w")
                .long("width")
                .help("map width")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("height")
                .required(true)
                .short("h")
                .long("height")
                .help("map height")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("name")
                .required(true)
                .short("n")
                .long("name")
                .help("map name")
                .takes_value(true)
                .index(1),
        )
        .author("Adrian Woźniak")
        .get_matches();

    let layers = matches.values_of("layers").unwrap().collect();
    let details = MapDetails::new(
        matches.value_of("width").unwrap().parse::<i32>().unwrap(),
        matches.value_of("height").unwrap().parse::<i32>().unwrap(),
        matches.value_of("name").unwrap().to_string(),
        &layers,
    );

    let path = "./assets/maps/".to_string() + &details.name + &".map".to_string();

    let mut buffer = String::new();
    buffer.push_str(&format!(
        "'meta w {:?} h {:?} {:?}\n\n",
        details.width, details.height, details.name
    ));
    layer_generator::generate_layers(&details, &mut buffer);
    write(path, buffer).unwrap();
}
