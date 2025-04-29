use crate::{
    io_utils::get_user_input,
    models::{Epic, Status, Story},
};

pub struct Prompts {
    pub create_epic: Box<dyn Fn() -> Epic>,
    pub create_story: Box<dyn Fn() -> Story>,
    pub delete_epic: Box<dyn Fn() -> bool>,
    pub delete_story: Box<dyn Fn() -> bool>,
    pub update_status: Box<dyn Fn() -> Option<Status>>,
}

impl Prompts {
    pub fn new() -> Self {
        Self {
            create_epic: Box::new(create_epic_prompt),
            create_story: Box::new(create_story_prompt),
            delete_epic: Box::new(delete_epic_prompt),
            delete_story: Box::new(delete_story_prompt),
            update_status: Box::new(update_status_prompt),
        }
    }
}

fn create_epic_prompt() -> Epic {
    println!("----------------------------");
    println!("Epic Name:");
    let mut name = get_user_input();
    println!("Epic Description:");
    let mut description = get_user_input();
    let epic = Epic::new(name, description);
    epic
}

fn create_story_prompt() -> Story {
    println!("----------------------------");
    println!("Story Name:");
    let mut name = get_user_input();
    println!("Story Description:");
    let mut description = get_user_input();
    let story = Story::new(name, description);
    story
}

fn delete_epic_prompt() -> bool {
    println!("----------------------------");
    println!("Are you sure you want to delete this Epic? All stories in this Epic will also be deleted [Y/n]:");
    let mut input = get_user_input();
    match input.trim() {
        "Y" => true,
        _ => false,
    }
}

fn delete_story_prompt() -> bool {
    println!("----------------------------");
    println!("Are you sure you want to delete this Story? [Y/n]:");
    let mut input = get_user_input();
    match input.trim() {
        "Y" => true,
        _ => false,
    }
}

fn update_status_prompt() -> Option<Status> {
    println!("----------------------------");
    println!("New Status (1 - OPEN, 2 - IN-PROGRESS, 3 - RESOLVED, 4 - CLOSED):");
    let mut input = get_user_input();
    if let Ok(valid_int) = input.trim().parse::<i32>() {
        return match valid_int {
            1 => Some(Status::Open),
            2 => Some(Status::InProgress),
            3 => Some(Status::Resolved),
            4 => Some(Status::Closed),
            _ => None,
        };
    }
    None
}
