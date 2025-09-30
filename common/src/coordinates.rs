use std::{
    marker::PhantomData,
    ops::{Add, Sub},
};

pub struct World;
pub struct Chunk;
pub struct Local;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position<T, S> {
    pub x: T,
    pub y: T,
    _space: PhantomData<S>,
}

pub type WorldPos = Position<i64, World>;
pub type ChunkPos = Position<i32, Chunk>;
pub type LocalPos = Position<u16, Local>;

impl WorldPos {
    /// Разделить мировые координаты на чанковые и локальные.
    pub fn split<const W: usize, const H: usize>(self) -> (ChunkPos, LocalPos) {
        let cx = self.x.div_euclid(W as i64) as i32;
        let cy = self.y.div_euclid(H as i64) as i32;
        let lx = self.x.rem_euclid(W as i64) as u16;
        let ly = self.y.rem_euclid(H as i64) as u16;
        (
            ChunkPos {
                x: cx,
                y: cy,
                _space: PhantomData,
            },
            LocalPos {
                x: lx,
                y: ly,
                _space: PhantomData,
            },
        )
    }
}

impl ChunkPos {
    pub fn join<const W: usize, const H: usize>(self, local: LocalPos) {}
}

/// Смещение
pub struct Offset {
    pub dx: i64,
    pub dy: i64,
}

impl Add<Offset> for WorldPos {
    type Output = WorldPos;

    #[inline]
    fn add(self, offset: Offset) -> Self::Output {
        WorldPos {
            x: self.x + offset.dx,
            y: self.y + offset.dy,
            _space: PhantomData,
        }
    }
}

impl Sub<Offset> for WorldPos {
    type Output = WorldPos;

    #[inline]
    fn sub(self, offset: Offset) -> Self::Output {
        WorldPos {
            x: self.x - offset.dx,
            y: self.y - offset.dy,
            _space: PhantomData,
        }
    }
}

impl Sub for WorldPos {
    type Output = Offset;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Offset {
            dx: self.x - rhs.x,
            dy: self.y - rhs.y,
        }
    }
}
