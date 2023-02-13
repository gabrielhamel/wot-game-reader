use crate::errors::GameReadError;
use crate::game_reader::GameReader;
use crate::localization::{LocalizationCatalog, Nation};
use crate::map::arena::BasePositions::Position1;
use crate::map::arena::ArenaMode::*;
use crate::map::arena::Team::{Team1, Team2};
use crate::map::arena::{BoundingBox, ArenaMode, Vector2, VehicleCamouflageKind};
use crate::map::reader::MapReader;
use std::collections::HashMap;
use std::env;

// Tests are run on an English client

fn get_reader() -> GameReader {
    let sources_path: String =
        env::var("SOURCES_PATH").expect("Environment variable SOURCES_PATH must be filled");
    GameReader::connect(&sources_path)
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

    assert_eq!(
        arena.vehicle_camouflage_kind()?,
        VehicleCamouflageKind::Summer
    );
    assert_eq!(
        arena.bounding_box()?,
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
    let gameplays: Vec<ArenaMode> = arena.available_modes();
    let gameplay_types = vec![Ctf, Assault2, Domination, Bootcamp, MapsTraining];
    for gameplay_type in &gameplay_types {
        assert_eq!(gameplays.contains(gameplay_type), true);
    }

    Ok(())
}

#[test]
fn localization_nation() -> Result<(), GameReadError> {
    let game_reader = get_reader();
    let localization = game_reader.localization();
    let ussr = localization.fetch(LocalizationCatalog::Nations, "ussr")?;
    assert_eq!(ussr, "U.S.S.R.");
    Ok(())
}

#[test]
fn localization_nationality() -> Result<(), GameReadError> {
    let game_reader = get_reader();
    let localization = game_reader.localization();
    let ussr = localization.fetch(LocalizationCatalog::Nations, "ussr/genetiveCase")?;
    assert_eq!(ussr, "Soviet");
    Ok(())
}

#[test]
fn arena_name() -> Result<(), GameReadError> {
    let game_reader = get_reader();
    let localization = game_reader.localization();
    let ussr = localization.fetch(LocalizationCatalog::Arenas, "01_karelia/name")?;
    assert_eq!(ussr, "Karelia");
    Ok(())
}

#[test]
fn map_name() -> Result<(), GameReadError> {
    let game_reader = get_reader();
    let map_reader = game_reader.maps();
    let map = map_reader.get_by_name("05_prohorovka")?;
    assert_eq!(map.display_name()?, "Prokhorovka");
    Ok(())
}

#[test]
fn tank_name() -> Result<(), GameReadError> {
    let game_reader = get_reader();
    let localization = game_reader.localization();
    let ussr = localization.fetch(
        LocalizationCatalog::Tanks(Nation::France),
        "F108_Panhard_EBR_105",
    )?;
    assert_eq!(ussr, "Panhard EBR 105");
    Ok(())
}

#[test]
fn tank_short_name() -> Result<(), GameReadError> {
    let game_reader = get_reader();
    let localization = game_reader.localization();
    let res = localization.fetch(LocalizationCatalog::Tanks(Nation::Japan), "J16_ST_B1");
    assert_eq!(res.is_err(), true);
    let res = localization.fetch(LocalizationCatalog::Tanks(Nation::Japan), "ST_B1")?;
    assert_eq!(res, "STB-1");
    Ok(())
}

#[test]
fn arena_merge() -> Result<(), GameReadError> {
    let game_reader = get_reader();
    let map = MapReader::from(&game_reader).get_by_name("01_karelia")?;
    let arena = map.arena()?;
    let assault = arena.get_mode(Assault);

    assert_eq!(assault.round_length()?, 600);
    assert_eq!(assault.winner_if_timeout, Some(Team1));
    assert_eq!(assault.winner_if_extermination, None);
    assert_eq!(
        assault.team_base_positions,
        Some(HashMap::from([
            (
                Team1,
                HashMap::from([(
                    Position1,
                    Some(Vector2 {
                        x: -278.80505,
                        y: 213.26526
                    })
                )])
            ),
            (Team2, HashMap::new())
        ]))
    );
    Ok(())
}
