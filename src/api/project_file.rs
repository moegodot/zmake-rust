use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::api::access_control::Visibility;

#[derive(Serialize, Deserialize, Debug)]
pub enum ScriptType{
    Building,
    Rule,
    Script
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BuildFile{
    file:Vec<String>,
    visibility: Visibility
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectFile{
    pub includes:Vec<BuildFile>,
    pub scripts:HashMap<ScriptType, String>,
}
