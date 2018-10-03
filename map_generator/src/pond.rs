use std::mem::replace;

pub enum PondType {
    SquarePond,
}

pub fn put_pond(stream: &mut Vec<Vec<i32>>, pond_type: &PondType, x: usize, y: usize) {
    match pond_type {
        PondType::SquarePond => {
            // X__
            // ___
            // ___
            put_tile(stream, x, y, 7);
            // ___
            // X__
            // ___
            put_tile(stream, x, y + 1, 8);
            // ___
            // ___
            // X__
            put_tile(stream, x, y + 2, 9);
            // ___
            // ___
            // _X_
            put_tile(stream, x + 1, y + 2, 10);
            // ___
            // ___
            // __X
            put_tile(stream, x + 2, y + 2, 11);
            // ___
            // __X
            // ___
            put_tile(stream, x + 2, y + 1, 12);
            // __X
            // ___
            // ___
            put_tile(stream, x + 2, y, 13);
            // _X_
            // ___
            // ___
            put_tile(stream, x + 1, y, 14);
            // ___
            // _X_
            // ___
            put_tile(stream, x + 1, y + 1, 15); // water
        }
    }
}

fn put_tile(stream: &mut Vec<Vec<i32>>, x: usize, y: usize, val: i32) {
    replace(&mut stream.get_mut(y).unwrap()[x], val);
}
