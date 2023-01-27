use crate::error::GameReadError;
use crate::map::arena::GameplayType::*;
use crate::map::arena::{BoundingBox, GameplayType, Vector2, VehicleCamouflageKing};
use crate::GameReader;
use std::env;

fn get_reader() -> GameReader {
    let game_path: String =
        env::var("GAME_PATH").expect("Environment variable GAME_PATH must be filled");
    let sources_path: String =
        env::var("SOURCES_PATH").expect("Environment variable SOURCES_PATH must be filled");
    GameReader::connect(&game_path, &sources_path)
}

#[test]
fn maps_fetching() -> Result<(), GameReadError> {
    let game_reader = get_reader();
    let map_reader = game_reader.maps();
    let maps = map_reader.list()?;

    for map in maps {
        match map.arena() {
            Err(e) => match e {
                GameReadError::ArenaDefinitionNotFound(_) => {
                    assert_eq!(map.is_development, true);
                }
                _ => {
                    return Err(e);
                }
            },
            _ => {}
        }
    }

    Ok(())
}

#[test]
fn arena_parsing() -> Result<(), GameReadError> {
    let game_reader = get_reader();
    let map_reader = game_reader.maps();

    let map = map_reader.get_by_name("05_prohorovka")?;

    assert_eq!(map.name, "05_prohorovka");

    let arena = map.arena()?;

    assert_eq!(arena.vehicle_camouflage_kind, VehicleCamouflageKing::Summer);
    assert_eq!(
        arena.bounding_box,
        BoundingBox {
            bottom_left: Vector2 {
                x: -500_f32,
                y: -500_f32
            },
            upper_right: Vector2 {
                x: 500_f32,
                y: 500_f32
            },
        }
    );
    let gameplays: Vec<GameplayType> = arena.gameplay_types.into_iter().map(|e| e.0).collect();
    let gameplay_types = vec![Ctf, Assault2, Domination, Bootcamp, MapsTraining];
    for gameplay_type in &gameplay_types {
        assert_eq!(gameplays.contains(gameplay_type), true);
    }

    Ok(())
}
