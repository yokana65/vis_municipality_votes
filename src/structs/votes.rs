use std::collections::HashMap;

use geo::Polygon;

#[derive(Debug)]
pub struct Vote {
    pub name: String,
    pub vote_records: Vec<VoteRecord>,
}

#[derive(Debug)]
pub struct VoteRecord {
    pub name_muni: String,
    pub votes: HashMap<String, i16>,
    pub geometry: Option<Polygon<f64>>,
}

