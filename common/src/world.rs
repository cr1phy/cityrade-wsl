use hashbrown::HashMap;
use noise::{NoiseFn, Perlin};
use rand::{rng, RngCore};
use rayon::iter::{
    IndexedParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator,
};
use std::{sync::Arc, vec::IntoIter};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::weather::Weather;

const CHUNK_WIDTH: usize = 64;
const CHUNK_HEIGHT: usize = 64;
const SCALE: f64 = 0.02;

type SizedChunk = Chunk<CHUNK_WIDTH, CHUNK_HEIGHT>;

pub struct World {
    id: Uuid,
    title: String,
    weather: Weather,
    map: WorldMap,
}

impl World {
    pub fn new(title: String) -> World {
        let mut rand = rng();
        World {
            id: Uuid::now_v7(),
            title,
            map: WorldMap::new(rand.next_u32()),
            weather: Weather::default(),
        }
    }

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

        c.tiles.par_iter_mut().enumerate().for_each(|(dy, row)| {
            row.iter_mut().enumerate().for_each(|(dx, tile)| {
                *tile = Tile::from(self.perlin.get([dx as f64 * SCALE, dy as f64 * SCALE]))
            })
        });

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
            tiles: vec![vec![Tile::Unknown; W]; H],
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
enum Tile {
    #[default]
    Unknown,
    Water,
    Grass,
    Mountain,
}

impl From<f64> for Tile {
    fn from(value: f64) -> Self {
        match value {
            _ => Self::Unknown,
        }
    }
}
