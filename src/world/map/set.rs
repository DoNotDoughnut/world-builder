use std::path::PathBuf;

use firecore_world::map::set::WorldMapSet;
use ahash::AHashMap as HashMap;
use crate::world::{SerializedMapSet, MapConfig};

pub fn load_map_set(root_path: &PathBuf, palette_sizes: &HashMap<u8, u16>, serialized_map_set: SerializedMapSet) -> (String, WorldMapSet) {

    println!("    Loading map set \"{}\"", serialized_map_set.identifier);

    let mut maps = Vec::new();

    for dir_string in serialized_map_set.dirs {
        let map_path = root_path.join(dir_string);
        for dir_entry in std::fs::read_dir(&map_path).unwrap_or_else(|err| panic!("Could not read map set directory at {:?} with error {}", map_path, err)) {
            let file = dir_entry.unwrap_or_else(|err| panic!("Could not read map set directory entry at {:?} with error {}", map_path, err)).path();
            if let Some(ext) = file.extension() {
                if ext == std::ffi::OsString::from("ron") {
                    let config: MapConfig = ron::from_str(
                        &std::fs::read_to_string(&file).unwrap_or_else(|err| panic!("Could not read map set configuration at {:?} to string with error {}", file, err))
                    ).unwrap_or_else(|err| panic!("Could not deserialize map set configuration at {:?} with error {}", file, err));
                    println!("        Loaded map set map \"{}\"", config.name);
                    maps.push(
                        super::load_map_from_config(&map_path, palette_sizes, config)
                    );
                }
            }
        }

        
        
    }

    (
        serialized_map_set.identifier,
        WorldMapSet {
            maps,
            ..Default::default()
        }
    )

}