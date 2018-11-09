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

#[cfg(test)]
mod tests {
    use super::*;

    struct Sizes {
        pub stream_len: usize,
        pub row_0: usize,
        pub row_1: usize,
        pub row_2: usize,
        pub row_3: usize,
        pub row_4: usize,
        pub row_5: usize,
        pub row_6: usize,
        pub row_7: usize,
    }

    fn build_stream() -> Vec<Vec<i32>> {
        vec![
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
        ]
    }

    #[test]
    fn it_should_not_change_size() {
        let mut stream = build_stream();
        let sizes: Sizes = {
            Sizes {
                stream_len: stream.len(),
                row_0: stream.get(0).unwrap().len(),
                row_1: stream.get(1).unwrap().len(),
                row_2: stream.get(2).unwrap().len(),
                row_3: stream.get(3).unwrap().len(),
                row_4: stream.get(4).unwrap().len(),
                row_5: stream.get(5).unwrap().len(),
                row_6: stream.get(6).unwrap().len(),
                row_7: stream.get(7).unwrap().len(),
            }
        };
        {
            put_pond(&mut stream, &PondType::SquarePond, 1, 2)
        };
        assert_eq!(stream.len(), sizes.stream_len);
        assert_eq!(stream.get(0).unwrap().len(), sizes.row_0);
        assert_eq!(stream.get(1).unwrap().len(), sizes.row_1);
        assert_eq!(stream.get(2).unwrap().len(), sizes.row_2);
        assert_eq!(stream.get(3).unwrap().len(), sizes.row_3);
        assert_eq!(stream.get(4).unwrap().len(), sizes.row_4);
        assert_eq!(stream.get(5).unwrap().len(), sizes.row_5);
        assert_eq!(stream.get(6).unwrap().len(), sizes.row_6);
        assert_eq!(stream.get(7).unwrap().len(), sizes.row_7);
    }

    // 0xN

    #[test]
    fn it_put_pond_in_given_coords() {
        let mut stream = build_stream();
        put_pond(&mut stream, &PondType::SquarePond, 1, 2);
        let a: [&[i32]; 8] = [
            stream.get(0).unwrap().as_slice(),
            stream.get(1).unwrap().as_slice(),
            stream.get(2).unwrap().as_slice(),
            stream.get(3).unwrap().as_slice(),
            stream.get(4).unwrap().as_slice(),
            stream.get(5).unwrap().as_slice(),
            stream.get(6).unwrap().as_slice(),
            stream.get(7).unwrap().as_slice(),
        ];
        assert_eq!(
            a,
            [
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 7, 14, 13, 0, 0, 0, 0],
                [0, 8, 15, 12, 0, 0, 0, 0],
                [0, 9, 10, 11, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
            ]
        );
    }
}
