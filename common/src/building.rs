use uuid::Uuid;

pub struct Building {
    id: Uuid,
    name: String,
    level: u8,
}

macro_rules! declare_building {
    () => {};
}
