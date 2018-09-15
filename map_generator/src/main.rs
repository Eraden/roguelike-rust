extern crate clap;
extern crate rand;

use rand::prelude::*;
use rand::thread_rng;
use std::fs::write;
use std::iter::repeat_with;
use clap::{Arg, App};

fn gen_rand(w: &i32, h: &i32, min: &i32) -> String {
    let mut rng = thread_rng();
    repeat_with(||
        repeat_with(|| (rng.gen_range(0, 5) + min).to_string())
            .take(*w as usize).collect::<Vec<String>>().join(" ")
    ).take(*h as usize).collect::<Vec<String>>().join("\n")
}

fn gen_section(name: &str, w: &i32, h: &i32, min: &i32) -> String {
    let mut buffer = String::new();
    buffer.push_str(&"'layer ");
    buffer.push_str(&name);
    buffer.push_str(&"\n");
    buffer.push_str(&gen_rand(w, h, min));
    buffer.push_str(&"\n\n".to_string());
    buffer
}

// cargo run -p map_generator -- first_map --width 40 --height 40 --layers animals ground1 ground2 ground3 players roofs plants
fn main() {
    let possible_layers = [
        "ground1", "ground2", "ground3",
        "animals", "plants", "players", "roofs"
    ];
    let matches = App::new("Map Generator")
        .version("1.0.0")
        .about("Simple generate map")
        .arg(Arg::with_name("layers")
            .short("l")
            .long("layers")
            .help("number of map layers")
            .takes_value(true)
            .possible_values(&possible_layers)
            .empty_values(true)
            .multiple(true)
        )
        .arg(Arg::with_name("width")
            .required(true)
            .short("w")
            .long("width")
            .help("map width")
            .takes_value(true)
        )
        .arg(Arg::with_name("height")
            .required(true)
            .short("h")
            .long("height")
            .help("map height")
            .takes_value(true)
        )
        .arg(Arg::with_name("name")
            .required(true)
            .short("n")
            .long("name")
            .help("map name")
            .takes_value(true)
            .index(1)
        )
        .author("Adrian Woźniak")
        .get_matches();

    let name = matches.value_of("name").unwrap();
    let width = matches.value_of("width").unwrap().parse::<i32>().unwrap();
    let height = matches.value_of("height").unwrap().parse::<i32>().unwrap();
    let layers: Vec<_> = matches.values_of("layers").unwrap().collect();

    let path = "./assets/maps/".to_string() + &name + &".map".to_string();

    let mut buffer = String::new();
    buffer.push_str(&format!("'meta w {:?} h {:?} {:?}\n\n", width, height, name));
    for name in layers {
        let min = match name {
            "ground1" => 0,
            "ground2" => 0,
            "ground3" => 0,
            "animals" => 10,
            "plants" => 20,
            "roofs" => 30,
            "players" => 60,
            _ => 0
        };
        buffer.push_str(&gen_section(&name, &width, &height, &min));
    }
    write(path, buffer).unwrap();
}
