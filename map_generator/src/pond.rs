pub enum PondType {
    SquarePond,
}

pub fn put_pond(stream: &mut Vec<Vec<i32>>, pond_type: &PondType, x: i32, y: i32) {
    match pond_type {
        PondType::SquarePond => {
            // X__
            // ___
            // ___
            stream
                .get_mut((y + 0) as usize)
                .unwrap()
                .insert((x + 0) as usize, 7);
            // ___
            // X__
            // ___
            stream
                .get_mut((y + 1) as usize)
                .unwrap()
                .insert((x + 0) as usize, 8);
            // ___
            // ___
            // X__
            stream
                .get_mut((y + 2) as usize)
                .unwrap()
                .insert((x + 0) as usize, 9);
            // ___
            // ___
            // _X_
            stream
                .get_mut((y + 2) as usize)
                .unwrap()
                .insert((x + 1) as usize, 10);
            // ___
            // ___
            // __X
            stream
                .get_mut((y + 2) as usize)
                .unwrap()
                .insert((x + 2) as usize, 11);
            // ___
            // __X
            // ___
            stream
                .get_mut((y + 1) as usize)
                .unwrap()
                .insert((x + 2) as usize, 12);
            // __X
            // ___
            // ___
            stream
                .get_mut((y + 0) as usize)
                .unwrap()
                .insert((x + 2) as usize, 13);
            // _X_
            // ___
            // ___
            stream
                .get_mut((y + 0) as usize)
                .unwrap()
                .insert((x + 1) as usize, 14);
            // ___
            // _X_
            // ___
            stream
                .get_mut((y + 1) as usize)
                .unwrap()
                .insert((x + 1) as usize, 15); // water
        }
    }
}
