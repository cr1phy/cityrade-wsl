mod building;
#[cfg(test)]
mod test;

use hashbrown::HashMap;
use noise::{NoiseFn, Perlin};
use std::{iter::Iterator, sync::Arc, vec::IntoIter};
use tokio::sync::RwLock;
use uuid::Uuid;

const CHUNK_WIDTH: usize = 64;
const CHUNK_HEIGHT: usize = 64;

type SizedChunk = Chunk<CHUNK_WIDTH, CHUNK_HEIGHT>;

pub struct World {
    id: Uuid,
    title: String,
    map: WorldMap,
}

impl World {
    pub async fn chunk_at(&mut self, coordinates: (i32, i32)) -> Arc<SizedChunk> {
        self.map.get_or_generate_chunk(coordinates).await
    }
}

#[derive(Debug, Default)]
pub struct WorldMap {
    perlin: Perlin,
    inner: RwLock<HashMap<(i32, i32), Arc<SizedChunk>>>,
}

impl WorldMap {
    pub fn new(seed: u32) -> Self {
        Self {
            perlin: Perlin::new(seed),
            inner: RwLock::new(HashMap::new()),
        }
    }

    pub async fn get_or_generate_chunk(&self, coords: (i32, i32)) -> Arc<SizedChunk> {
        if let Some(existed_chunk) = self.inner.read().await.get(&coords).cloned() {
            return existed_chunk;
        }

        let new = Arc::new(self.generate_chunk());

        let mut m = self.inner.write().await;
        Arc::clone(m.entry(coords).or_insert_with(|| Arc::clone(&new)))
    }

    fn generate_chunk(&self) -> SizedChunk {
        let mut c = SizedChunk::new();

        for r in c {
            for t in r {
                *t = Tile::UNKNOWN;
            }
        }

        c
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Chunk<const W: usize, const H: usize> {
    tiles: Vec<Vec<Tile>>,
}

impl<const W: usize, const H: usize> Chunk<W, H> {
    pub fn new() -> Self {
        Self {
            tiles: vec![vec![Tile::UNKNOWN; W]; H],
        }
    }
}

impl<const W: usize, const H: usize> IntoIterator for Chunk<W, H> {
    type Item = Vec<Tile>;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.tiles.into_iter()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Tile {
    UNKNOWN,
}
