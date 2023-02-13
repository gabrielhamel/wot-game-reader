use crate::errors::GameReadError;
use crate::game_reader::GameReader;
use crate::map::arena::Team::{Team1, Team2};
use crate::map::Map;
use merge::Merge;
use serde::{de, Deserialize, Deserializer, Serialize};
use serde_xml_rs as xml;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::str::FromStr;

#[derive(Debug, Serialize, PartialEq, Copy, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

fn deserialize_float<'de, D>(s: &str) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    match f32::from_str(s) {
        Ok(number) => Ok(number),
        Err(_) => Err(de::Error::custom(format!("Invalid coord in {}", s))),
    }
}

impl<'de> Deserialize<'de> for Vector2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let buf = String::deserialize(deserializer)?;
        let numbers: Vec<&str> = buf.split(" ").collect();

        if numbers.len() != 2 {
            return Err(de::Error::custom(format!("Invalid 2d coords in {}", buf)));
        }

        Ok(Vector2 {
            x: deserialize_float::<D>(numbers[0])?,
            y: deserialize_float::<D>(numbers[1])?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BoundingBox {
    pub bottom_left: Vector2,
    pub upper_right: Vector2,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum BasePositions {
    Position1,
    Position2,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Team {
    Team1,
    Team2,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Positions {
    #[serde(rename = "$value")]
    pub positions: Option<Vec<Vector2>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TeamSpawnPoints {
    pub team1: Positions,
    pub team2: Positions,
}

fn team_id_deserializer<'de, D>(deserializer: D) -> Result<Option<Team>, D::Error>
where
    D: Deserializer<'de>,
{
    let id: Option<i32> = Deserialize::deserialize(deserializer)?;
    Ok(match id {
        Some(team_id) => match team_id {
            1 => Some(Team1),
            2 => Some(Team2),
            _ => None,
        },
        None => None,
    })
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Merge)]
#[serde(rename_all = "camelCase")]
pub struct ArenaModeDesc {
    #[merge(strategy = merge::option::overwrite_none)]
    pub team_base_positions: Option<HashMap<Team, HashMap<BasePositions, Option<Vector2>>>>,

    #[merge(strategy = merge::option::overwrite_none)]
    pub team_spawn_points: Option<TeamSpawnPoints>,

    #[merge(strategy = merge::option::overwrite_none)]
    #[serde(default, deserialize_with = "team_id_deserializer")]
    pub winner_if_timeout: Option<Team>,

    #[merge(strategy = merge::option::overwrite_none)]
    #[serde(default, deserialize_with = "team_id_deserializer")]
    pub winner_if_extermination: Option<Team>,

    #[merge(strategy = merge::option::overwrite_none)]
    round_length: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub enum VehicleCamouflageKind {
    Summer,
    Winter,
    Desert,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ArenaMode {
    Ctf,
    Ctf2,
    Ctf30x30, // Grand battle
    Domination,
    Domination2,
    Domination30x30,
    Assault,
    Assault2,
    Bootcamp,
    MapsTraining,
    Sandbox,
    Comp7, // 7v7 Ranked
    Epic,  // Front line
}

impl fmt::Display for ArenaMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Merge)]
#[serde(rename_all = "camelCase")]
pub struct ArenaDefinition {
    #[merge(strategy = merge::option::overwrite_none)]
    bounding_box: Option<BoundingBox>,

    #[merge(strategy = merge::hashmap::intersection)]
    gameplay_types: HashMap<ArenaMode, ArenaModeDesc>,

    #[merge(strategy = merge::option::overwrite_none)]
    vehicle_camouflage_kind: Option<VehicleCamouflageKind>,

    #[merge(strategy = merge::option::overwrite_none)]
    round_length: Option<i32>,

    #[merge(strategy = merge::option::overwrite_none)]
    #[serde(default, deserialize_with = "team_id_deserializer")]
    pub winner_if_timeout: Option<Team>,

    #[merge(strategy = merge::option::overwrite_none)]
    #[serde(default, deserialize_with = "team_id_deserializer")]
    pub winner_if_extermination: Option<Team>,
}

impl ArenaDefinition {
    pub fn parse(game_reader: &GameReader, map: &Map) -> Result<ArenaDefinition, GameReadError> {
        let default = File::open(
            game_reader
                .sources_path()
                .join("res")
                .join("scripts")
                .join("arena_defs")
                .join("_default_.xml"),
        )?;
        let default_def: ArenaDefinition = xml::from_reader(default)?;
        if map.is_development {
            return Ok(default_def);
        }

        let map = File::open(
            game_reader
                .sources_path()
                .join("res")
                .join("scripts")
                .join("arena_defs")
                .join(format!("{}.xml", map.name)),
        )?;
        let mut map_def: ArenaDefinition = xml::from_reader(map)?;
        map_def.merge(default_def);

        // Merge shared fields between different gameplay and default arena
        for (_, mut gameplay) in &mut map_def.gameplay_types {
            if gameplay.winner_if_extermination == None {
                gameplay.winner_if_extermination = map_def.winner_if_extermination;
            }
            if gameplay.winner_if_timeout == None {
                gameplay.winner_if_timeout = map_def.winner_if_timeout;
            }
            if gameplay.round_length == None {
                gameplay.round_length = map_def.round_length;
            }
        }

        Ok(map_def)
    }

    pub fn vehicle_camouflage_kind(&self) -> Result<VehicleCamouflageKind, GameReadError> {
        self.vehicle_camouflage_kind
            .ok_or(GameReadError::ArenaKeyNotFound(String::from(
                "vehicle_camouflage_kind",
            )))
    }

    pub fn bounding_box(&self) -> Result<BoundingBox, GameReadError> {
        self.bounding_box
            .ok_or(GameReadError::ArenaKeyNotFound(String::from(
                "bounding_box",
            )))
    }

    pub fn available_modes(&self) -> Vec<ArenaMode> {
        self.gameplay_types.keys().cloned().collect()
    }

    pub fn get_mode(&self, mode: ArenaMode) -> &ArenaModeDesc {
        &self.gameplay_types[&mode]
    }
}

impl ArenaModeDesc {
    pub fn round_length(&self) -> Result<i32, GameReadError> {
        self.round_length
            .ok_or(GameReadError::ArenaKeyNotFound(String::from(
                "round_length",
            )))
    }
}
