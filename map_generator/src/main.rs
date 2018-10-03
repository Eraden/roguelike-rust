extern crate clap;
extern crate rand;

pub mod pond;

use clap::{App, Arg};
use rand::prelude::*;
use rand::thread_rng;
use std::fs::write;
use std::iter::repeat_with;

use pond::*;

fn gen_rand(w: &i32, h: &i32, min: &i32, max: &i32) -> Vec<Vec<i32>> {
    let mut rng = thread_rng();
    repeat_with(|| {
        repeat_with(|| rng.gen_range(*min, *max))
            .take(*w as usize)
            .collect::<Vec<i32>>()
    }).take(*h as usize)
    .collect::<Vec<Vec<i32>>>()
}

fn put_header(name: &str, buffer: &mut String) {
    buffer.push_str(&"'layer ");
    buffer.push_str(&name);
    buffer.push_str(&"\n");
}

fn put_tail(buffer: &mut String) {
    buffer.push_str(&"\n\n".to_string());
}

fn stringify_stream(stream: &Vec<Vec<i32>>) -> String {
    stream
        .iter()
        .map(|a| {
            a.iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        }).collect::<Vec<String>>()
        .join("\n")
}

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
        ).arg(
            Arg::with_name("width")
                .required(true)
                .short("w")
                .long("width")
                .help("map width")
                .takes_value(true),
        ).arg(
            Arg::with_name("height")
                .required(true)
                .short("h")
                .long("height")
                .help("map height")
                .takes_value(true),
        ).arg(
            Arg::with_name("name")
                .required(true)
                .short("n")
                .long("name")
                .help("map name")
                .takes_value(true)
                .index(1),
        ).author("Adrian Woźniak")
        .get_matches();

    let name = matches.value_of("name").unwrap();
    let width = matches.value_of("width").unwrap().parse::<i32>().unwrap();
    let height = matches.value_of("height").unwrap().parse::<i32>().unwrap();
    let layers: Vec<_> = matches.values_of("layers").unwrap().collect();

    let path = "./assets/maps/".to_string() + &name + &".map".to_string();

    let mut buffer = String::new();
    buffer.push_str(&format!(
        "'meta w {:?} h {:?} {:?}\n\n",
        width, height, name
    ));
    for name in layers {
        let min = match name {
            "ground1" => 0,
            "ground2" => 0,
            "ground3" => 0,
            "animals" => 30,
            "plants" => 50,
            "roofs" => 70,
            "players" => 90,
            _ => 0,
        };
        let max = match name {
            "ground1" => min.clone() + 6,
            "ground2" => min.clone() + 6,
            "ground3" => min.clone() + 6,
            "animals" => min.clone() + 5,
            "plants" => min.clone() + 5,
            "roofs" => min.clone() + 5,
            "players" => min.clone() + 5,
            _ => min.clone() + 5,
        };
        let mut section_buffer = String::new();
        put_header(&name, &mut section_buffer);
        let mut section_stream = gen_rand(&width, &height, &min, &max);

        match name {
            "ground1" => {
                put_pond(&mut section_stream, &PondType::SquarePond, 1, 1);
            }
            "ground2" => {
                put_pond(&mut section_stream, &PondType::SquarePond, 4, 4);
            }
            "ground3" => {
                put_pond(&mut section_stream, &PondType::SquarePond, 4, 1);
            }
            "animals" => {}
            "plants" => {}
            "roofs" => {}
            "players" => {}
            _ => {}
        };

        section_buffer.push_str(stringify_stream(&section_stream).as_str());
        put_tail(&mut section_buffer);
        buffer.push_str(section_buffer.as_str());
    }
    write(path, buffer).unwrap();
}
