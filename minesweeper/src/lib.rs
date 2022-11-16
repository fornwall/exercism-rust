pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result = Vec::new();

    for (row_idx, row) in minefield.iter().enumerate() {
        let mut new_row = String::new();
        for (col_idx, col) in row.bytes().enumerate() {
            if col == b'*' {
                new_row.push(col as char);
                continue;
            }

            let mut bombs: u8 = 0;
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if (dx, dy) != (0, 0) {
                        let y = (row_idx as i32 + dy) as usize;
                        let x = (col_idx as i32 + dx) as usize;
                        bombs += minefield
                            .get(y)
                            .map(|row| row.as_bytes().get(x).map_or(0, |c| u8::from(*c == b'*')))
                            .unwrap_or_default();
                    }
                }
            }

            new_row.push(if bombs == 0 {
                ' '
            } else {
                (b'0' + bombs) as char
            });
        }
        result.push(new_row);
    }

    result
}
