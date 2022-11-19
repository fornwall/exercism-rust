#[derive(Copy, Clone, Debug)]
pub struct ChessPosition {
    rank: u8,
    file: u8,
}

#[derive(Debug, Copy, Clone)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    #[must_use]
    pub const fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 0 || file < 0 || rank > 7 || file > 7 {
            None
        } else {
            Some(Self {
                rank: rank as u8,
                file: file as u8,
            })
        }
    }
}

impl Queen {
    #[must_use]
    pub const fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    #[must_use]
    pub const fn can_attack(&self, other: &Self) -> bool {
        self.position.file == other.position.file
            || self.position.rank == other.position.rank
            || (self.position.rank as i8 - other.position.rank as i8).abs()
                == (self.position.file as i8 - other.position.file as i8).abs()
    }
}
