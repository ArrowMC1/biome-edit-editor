#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MainMap {
    Main,
    ShadedRelief,
    Heightmap
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NoiseMap {
    Temperature,
    Humidity,
    Continental,
    Erosion,
    Weirdness
}