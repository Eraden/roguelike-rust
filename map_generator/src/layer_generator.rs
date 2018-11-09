use pond::*;
use rand::prelude::*;
use rand::thread_rng;
use std::iter::repeat_with;

pub struct MapDetails<'a> {
    pub width: i32,
    pub height: i32,
    pub name: String,
    pub layers: Vec<&'a str>,
}

impl<'a> MapDetails<'a> {
    pub fn new(width: i32, height: i32, name: String, layers: &Vec<&'a str>) -> Self {
        Self {
            width,
            height,
            name,
            layers: layers.to_vec(),
        }
    }
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

fn resolve_min(name: &str) -> i32 {
    match name {
        "ground1" => 0,
        "ground2" => 0,
        "ground3" => 0,
        "animals" => 30,
        "plants" => 50,
        "roofs" => 70,
        "players" => 90,
        _ => 0,
    }
}

fn resolve_max(name: &str, min: &i32) -> i32 {
    match name {
        "ground1" => min.clone() + 6,
        "ground2" => min.clone() + 6,
        "ground3" => min.clone() + 6,
        "animals" => min.clone() + 5,
        "plants" => min.clone() + 5,
        "roofs" => min.clone() + 5,
        "players" => min.clone() + 5,
        _ => min.clone() + 5,
    }
}

fn generate_layer(name: &str, details: &MapDetails) -> String {
    let mut section_buffer = String::new();
    let min: i32 = resolve_min(name);
    let max: i32 = resolve_max(name, &min);
    put_header(&name, &mut section_buffer);
    let mut section_stream = gen_rand(&details.width, &details.height, &min, &max);

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
    section_buffer
}

pub fn generate_layers(details: &MapDetails, buffer: &mut String) {
    for name in &details.layers {
        let section_buffer = generate_layer(name, details);
        buffer.push_str(section_buffer.as_str());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_resolve_min() {
        assert_eq!(resolve_min("ground1"), 0);
        assert_eq!(resolve_min("ground2"), 0);
        assert_eq!(resolve_min("ground3"), 0);
        assert_eq!(resolve_min("animals"), 30);
        assert_eq!(resolve_min("plants"), 50);
        assert_eq!(resolve_min("roofs"), 70);
        assert_eq!(resolve_min("players"), 90);
        assert_eq!(resolve_min("invalid"), 0);
        assert_eq!(resolve_min("-"), 0);
        assert_eq!(resolve_min("+"), 0);
    }

    #[test]
    fn it_resolve_max() {
        assert_eq!(resolve_max("ground1", &11), 11 + 6);
        assert_eq!(resolve_max("ground2", &11), 11 + 6);
        assert_eq!(resolve_max("ground3", &11), 11 + 6);
        assert_eq!(resolve_max("animals", &11), 11 + 5);
        assert_eq!(resolve_max("plants", &11), 11 + 5);
        assert_eq!(resolve_max("roofs", &11), 11 + 5);
        assert_eq!(resolve_max("players", &11), 11 + 5);
        assert_eq!(resolve_max("", &11), 11 + 5);
        assert_eq!(resolve_max("invalid", &11), 11 + 5);
        assert_eq!(resolve_max("-", &11), 11 + 5);
        assert_eq!(resolve_max("+", &11), 11 + 5);
    }
}
