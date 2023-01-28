use crate::errors::GameReadError;
use serde::{de, Deserialize, Deserializer, Serialize};
use serde_xml_rs as xml;
use std::collections::HashMap;
use std::fs::File;
use std::str::FromStr;
use crate::GameReader;

#[derive(Debug, Serialize, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum Team {
    Team1,
    Team2,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Positions {
    #[serde(rename = "$value")]
    positions: Option<Vec<Vector2>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TeamSpawnPoints {
    pub team1: Positions,
    pub team2: Positions,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Gameplay {
    pub team_base_positions: Option<HashMap<Team, HashMap<BasePositions, Option<Vector2>>>>,
    pub team_spawn_points: Option<TeamSpawnPoints>,
    pub winner_if_timeout: Option<i32>,
    pub winner_if_extermination: Option<i32>,
    pub round_length: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum VehicleCamouflageKing {
    Summer,
    Winter,
    Desert,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum GameplayType {
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ArenaDefinition {
    pub bounding_box: Option<BoundingBox>,
    pub gameplay_types: HashMap<GameplayType, Gameplay>,
    pub vehicle_camouflage_kind: Option<VehicleCamouflageKing>,
}

impl ArenaDefinition {
    pub fn parse(game_reader: &GameReader, name: &str) -> Result<ArenaDefinition, GameReadError> {
        let path = game_reader
            .sources_path
            .join("res")
            .join("scripts")
            .join("arena_defs")
            .join(format!("{}.xml", name));
        let file = File::open(path)?;
        let def: ArenaDefinition = xml::from_reader(file)?;
        Ok(def)
    }
}
