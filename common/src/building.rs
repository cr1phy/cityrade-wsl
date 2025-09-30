use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Building {
    id: Uuid,
    name: String,
    level: u8,
    #[serde(flatten)]
    inner: BuildingType,
    details: SmallVec<[Detail; 6]>,
    extra: Extra,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BuildingType {
    Residential,
    Public,
    Commercial,
    Industrial,
    Transport,
    Argicultural,
    Religious,
    Special,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Detail {
    Tag(TagId),
    Capacity { total: u32 },
    Residents { current: u32 },
    Employees { count: u16 },
    Visitors { per_day: u32 },
    Revenue { per_day: i64 },
    Throughput { per_hour: u32 },
    Pollution { ppm: u16 },
    Custom { kind: u32, blob: Vec<u8> },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagId(pub u32);

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PropKey(pub u32);

#[derive(Debug, Serialize, Deserialize)]
pub enum Value {
    UnsignedInt32(u32),
    UnsignedInt16(u16),
    Int64(i64),
    Float32(f32),
    Boolean(bool),
    Text(String),
    Blob(Vec<u8>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Extra {
    pub props: SmallVec<[(PropKey, Value); 8]>,
}

impl Extra {
    pub fn get(&self, key: PropKey) -> Option<&Value> {
        self.props
            .iter()
            .find_map(|(propkey, value)| (*propkey == key).then_some(value))
    }

    pub fn set(&mut self, key: PropKey, value: Value) {
        if let Some(slot) = self.props.iter_mut().find(|(propkey, _)| *propkey == key) {
            slot.1 = value;
        } else {
            self.props.push((key, value));
        }
    }
}
