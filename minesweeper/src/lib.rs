#[must_use]
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    minefield
        .iter()
        .enumerate()
        .map(|(row_idx, row)| {
            row.bytes()
                .enumerate()
                .map(|(col_idx, col)| {
                    if col == b'*' {
                        col as char
                    } else {
                        let mut bombs: u8 = 0;
                        for dx in -1..=1 {
                            for dy in -1..=1 {
                                if (dx, dy) != (0, 0) {
                                    let y = (row_idx as i32 + dy) as usize;
                                    let x = (col_idx as i32 + dx) as usize;
                                    bombs += minefield
                                        .get(y)
                                        .map(|row| {
                                            row.as_bytes()
                                                .get(x)
                                                .map_or(0, |c| u8::from(*c == b'*'))
                                        })
                                        .unwrap_or_default();
                                }
                            }
                        }

                        if bombs == 0 {
                            ' '
                        } else {
                            (b'0' + bombs) as char
                        }
                    }
                })
                .collect()
        })
        .collect()
}
