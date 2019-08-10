use super::models::*;
use std::fmt::{Display, Formatter, Result};


impl Display for BuildingType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            BuildingType::BlueFlowers =>
                write!(f, "blue flowers"),
            BuildingType::RedFlowers =>
                write!(f, "red flower field"),
            BuildingType::Tree =>
                write!(f, "tree"),
            BuildingType::BundlingStation =>
                write!(f, "bundling station"),
            BuildingType::SawMill =>
                write!(f, "saw mill"),
        }
    }

}