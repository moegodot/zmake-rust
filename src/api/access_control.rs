use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Visibility{
    VisibleTo(Vec<String>),
    Private,
    Public
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TransitiveLevel{
    Interface,
    Public,
    Private
}
