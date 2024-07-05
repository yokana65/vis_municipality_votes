use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

use anyhow::Result;
use geo::Polygon;
use geojson::{Feature, FeatureCollection, GeoJson, Geometry};
use serde_json::Value as JsonValue;

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

impl Vote {
    pub fn write_geojson(&self) -> Result<()> {
        let filename = self.name.as_str();

        let features: Vec<Feature> = self
            .vote_records
            .iter()
            .map(|record| {
                // Create properties
                let mut properties = serde_json::Map::new();
                properties.insert(
                    "name_muni".to_string(),
                    JsonValue::String(record.name_muni.clone()),
                );

                // Add vote counts to properties
                for (party, count) in &record.votes {
                    properties.insert(party.clone(), JsonValue::Number((*count).into()));
                }

                // Create geometry
                let geometry = record.geometry.as_ref().map(|polygon| {
                    Geometry::new(geojson::Value::Polygon(vec![polygon
                        .exterior()
                        .coords()
                        .map(|coord| vec![coord.x, coord.y])
                        .collect()]))
                });

                // Create feature
                Feature {
                    bbox: None,
                    geometry,
                    id: None,
                    properties: Some(properties),
                    foreign_members: None,
                }
            })
            .collect();

        let feature_collection = FeatureCollection {
            bbox: None,
            features,
            foreign_members: None,
        };

        // Convert FeatureCollection to GeoJson
        let geojson = GeoJson::FeatureCollection(feature_collection);

        // Serialize to a JSON string
        let geojson_string = serde_json::to_string_pretty(&geojson)?;

        // Write to a file
        let path = "./data/".to_string() + filename;
        dbg!(&path);
        let mut file = File::create(path)?;
        file.write_all(geojson_string.as_bytes()).expect("Failed to write GeoJson");

        Ok(())
    }
}
