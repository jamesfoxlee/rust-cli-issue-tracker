use clearscreen;

use std::rc::Rc;

mod models;

mod db;
use db::*;

mod ui;

mod io_utils;
use io_utils::*;

mod navigator;
use navigator::*;

const DB_PATH: &str = r"./data/db.json";

fn main() {
    println!("Initialising CLI Issue Tracker...");
    let db_path = String::from(DB_PATH);
    let db = db::JiraDatabase::new(db_path);
    let rc_db = Rc::new(db);
    let mut navigator = navigator::Navigator::new(rc_db);

    loop {
        clearscreen::clear().unwrap();
        // TODO: implement the following functionality:
        // 1. get current page from navigator. If there is no current page exit the loop
        if let Some(page) = navigator.get_current_page() {
            // 2. render page
            match page.draw_page() {
                Ok(_) => {
                    // 3. get user input
                    let input = get_user_input();
                    match page.handle_input(input.trim()) {
                        Ok(action) => {
                            if let Some(action) = action {
                                if let Err(e) = navigator.handle_action(action) {
                                    println!("ERROR: navigator.handle_action: {e}");
                                }
                            }
                        }
                        Err(e) => println!("ERROR: handle_input() {}", e),
                    }
                }
                Err(e) => {
                    println!("ERROR: draw_page() {e}\nPress a key to exit");
                    wait_for_key_press();
                }
            }
        } else {
            break;
        }
        // 4. pass input to page's input handler
        // 5. if the page's input handler returns an action let the navigator process the action
    }
}
