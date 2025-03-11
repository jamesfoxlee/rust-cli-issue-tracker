use std::collections::HashMap;

pub type Id = u32;
pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed
}

pub struct Epic {
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<Id>,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            status: Status::Open,
            stories: Vec::new(),
        }
    }
}

pub struct Story {
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            status: Status::Open,
        }
    }
}

pub struct DBState {
    last_item_id: Id,
    epics: HashMap<Id, Epic>,
    stories: HashMap<Id, Story>,
}