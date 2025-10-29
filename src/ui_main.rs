use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};

mod encrypt;
mod vault;

use vault::Vault;

#[derive(PartialEq)]
enum MenuState {
    LogInMenu,
    InitScreen,
    SelectMenu,
    AddUpdateScreen
}

#[derive(PartialEq)]
enum LogInState {
    In,
    Out
}

#[derive(PartialEq)]
enum AddOrUpdate {
    Add,
    Update
}

fn conf() -> Conf {
    Conf {
        window_title: "RsPass".to_owned(),
        window_width: 600,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut login_state = LogInState::Out;
    let mut menu_state = MenuState::LogInMenu;
    let mut add_or_update = AddOrUpdate::Add;

    let mut vault: Option<Vault> = None;
    let mut master_password = String::new();
    let mut username = String::new();
    let mut password = String::new();
    let mut combobox = 0;
    let mut list_of_passwords: Vec<String> = vec!["None".to_string()];
    let mut error_message = String::new();
    let mut success_message = String::new();

    loop {
        clear_background(GRAY);

        match menu_state {
            MenuState::LogInMenu => {
                root_ui().window(hash!(), vec2(100.0, 100.0), vec2(400.0, 400.0), |ui| {
                    ui.label(None, "Master Password:");
                    widgets::InputText::new(hash!()).password(true).size(vec2(260.0, 30.0)).ui(ui, &mut master_password);

                    if !error_message.is_empty() {
                        ui.label(None, &format!("Error: {}", error_message));
                    }

                    if widgets::Button::new("Create Vault").ui(ui) {
                        error_message.clear();
                        menu_state = MenuState::InitScreen;
                    }

                    if widgets::Button::new("Log In").ui(ui) {
                        error_message.clear();

                        if !master_password.is_empty() {
                            match Vault::load(&master_password) {
                                Ok(loaded_vault) => {
                                    vault = Some(loaded_vault);
                                    login_state = LogInState::In;
                                    menu_state = MenuState::SelectMenu;
                                    
                                    // Update the list of passwords
                                    if let Some(ref v) = vault {
                                        list_of_passwords = v.list_services().iter().map(|s| s.to_string()).collect();

                                        if list_of_passwords.is_empty() {
                                            list_of_passwords = vec!["None".to_string()];
                                        }
                                    }

                                    combobox = 0;
                                    username.clear();
                                    password.clear();
                                }

                                Err(e) => {
                                    error_message = format!("Failed to load vault: {}", e);
                                }
                            }
                        } else {
                            error_message = "Master password cannot be empty".to_string();
                        }
                    }    
                });
            }

            MenuState::InitScreen => {
                root_ui().window(hash!(), vec2(100.0, 100.0), vec2(400.0, 400.0), |ui| {
                    ui.label(None, "Create New Vault");
                    ui.label(None, "Master Password:");
                    widgets::InputText::new(hash!()).password(true).size(vec2(260.0, 30.0)).ui(ui, &mut master_password);

                    if !error_message.is_empty() {
                        ui.label(None, &format!("Error: {}", error_message));
                    }

                    if widgets::Button::new("Create").ui(ui) {
                        error_message.clear();

                        if !master_password.is_empty() {
                            match Vault::create_new(&master_password) {
                                Ok(_) => {
                                    menu_state = MenuState::LogInMenu;
                                    master_password.clear();
                                }

                                Err(e) => {
                                    error_message = format!("Failed to create vault: {}", e);
                                }
                            }
                        } else {
                            error_message = "Master password cannot be empty".to_string();
                        }
                    }

                    if widgets::Button::new("Cancel").ui(ui) {
                        error_message.clear();
                        master_password.clear();
                        menu_state = MenuState::LogInMenu;
                    }
                });
            }

            MenuState::SelectMenu => {
                root_ui().window(hash!(), vec2(100.0, 100.0), vec2(400.0, 400.0), |ui| {
                    if login_state == LogInState::In {
                        if widgets::Button::new("Log Out").ui(ui) {
                            vault = None;
                            login_state = LogInState::Out;
                            menu_state = MenuState::LogInMenu;
                            master_password.clear();
                            username.clear();
                            password.clear();
                            list_of_passwords = vec!["None".to_string()];
                            combobox = 0;
                            error_message.clear();
                            success_message.clear();
                        }

                        if !success_message.is_empty() {
                            ui.label(None, &format!("Success: {}", success_message));
                        }

                        if !error_message.is_empty() {
                            ui.label(None, &format!("Error: {}", error_message));
                        }

                        // List of passwords
                        let list_refs: Vec<&str> = list_of_passwords.iter().map(|s| s.as_str()).collect();
                        ui.combo_box(hash!(), ": Passwords in the Vault", &list_refs, &mut combobox);

                        ui.label(None, &username);
                        ui.label(None, &password);

                        if widgets::Button::new("Show").ui(ui) {
                            // Set username and password variables based on selection in combobox
                            if let Some(ref v) = vault {
                                if !list_of_passwords.is_empty() && list_of_passwords[0] != "None" {
                                    let selected_service = &list_of_passwords[combobox];
                                    username = selected_service.clone();
                                    
                                    if let Some(pwd) = v.get_password(selected_service) {
                                        password = pwd.to_string();
                                    } else {
                                        password = "Not found".to_string();
                                    }
                                }
                            }
                        }

                        if widgets::Button::new("Clear").ui(ui) {
                            error_message.clear();
                            success_message.clear();
                            combobox = 0;
                            username.clear();
                            password.clear();
                        }

                        if widgets::Button::new("Add").ui(ui) {
                            error_message.clear();
                            success_message.clear();
                            add_or_update = AddOrUpdate::Add;
                            username.clear();
                            password.clear();
                            menu_state = MenuState::AddUpdateScreen;
                        }

                        if widgets::Button::new("Update").ui(ui) {
                            error_message.clear();
                            success_message.clear();

                            // Set username and password variables based on selection in combobox
                            if let Some(ref v) = vault {
                                if !list_of_passwords.is_empty() && list_of_passwords[0] != "None" {
                                    let selected_service = &list_of_passwords[combobox];
                                    username = selected_service.clone();
                                    
                                    if let Some(pwd) = v.get_password(selected_service) {
                                        password = pwd.to_string();
                                    } else {
                                        password = "Not found".to_string();
                                    }
                                }
                            }

                            if !list_of_passwords.is_empty() && list_of_passwords[0] != "None" {
                                add_or_update = AddOrUpdate::Update;
                                menu_state = MenuState::AddUpdateScreen;
                            } else {
                                error_message = "No passwords to update".to_string();
                            }
                        }

                        if widgets::Button::new("Remove").ui(ui) {
                            error_message.clear();
                            success_message.clear();

                            if let Some(ref mut v) = vault {
                                if !list_of_passwords.is_empty() && list_of_passwords[0] != "None" {
                                    let selected_service = list_of_passwords[combobox].clone();

                                    match v.remove_password(&selected_service) {
                                        Ok(removed) => {
                                            if removed {
                                                match v.save(&master_password) {
                                                    Ok(_) => {
                                                        // Update the list of passwords
                                                        success_message = format!("Password removed for '{}'", selected_service);
                                                        list_of_passwords = v.list_services().iter().map(|s| s.to_string()).collect();

                                                        if list_of_passwords.is_empty() {
                                                            list_of_passwords = vec!["None".to_string()];
                                                        }

                                                        combobox = 0;
                                                        username.clear();
                                                        password.clear();
                                                    }

                                                    Err(e) => {
                                                        error_message = format!("Failed to save: {}", e);
                                                    }
                                                }
                                            } else {
                                                error_message = format!("No password found for '{}'", selected_service);
                                            }
                                        }

                                        Err(e) => {
                                            error_message = format!("Failed to remove: {}", e);
                                        }
                                    }
                                } else {
                                error_message = "No passwords to delete".to_string();
                            }
                            }
                        }
                    }
                });
            }

            MenuState::AddUpdateScreen => {
                root_ui().window(hash!(), vec2(100.0, 100.0), vec2(400.0, 400.0), |ui| {
                    if login_state == LogInState::In {
                        if widgets::Button::new("Log Out").ui(ui) {
                            vault = None;
                            login_state = LogInState::Out;
                            menu_state = MenuState::LogInMenu;
                            master_password.clear();
                            username.clear();
                            password.clear();
                            list_of_passwords = vec!["None".to_string()];
                            combobox = 0;
                            error_message.clear();
                            success_message.clear();
                        }

                        if !error_message.is_empty() {
                            ui.label(None, &format!("Error: {}", error_message));
                        }

                        // Text fields: Username, Password
                        ui.label(None, "Username:");
                        widgets::InputText::new(hash!()).size(vec2(260.0, 30.0)).ui(ui, &mut username);

                        ui.label(None, "Password:");
                        widgets::InputText::new(hash!()).password(true).size(vec2(260.0, 30.0)).ui(ui, &mut password);

                        
                        match add_or_update {
                            AddOrUpdate::Add => {
                                if widgets::Button::new("Add").ui(ui) {
                                    error_message.clear();

                                    if username.is_empty() || password.is_empty() {
                                        error_message = "Service name and password cannot be empty".to_string();
                                    } else {
                                        if let Some(ref mut v) = vault {
                                            match v.add_password(&username, &password) {
                                                Ok(_) => {
                                                    match v.save(&master_password) {
                                                        Ok(_) => {
                                                            // Update the list of passwords
                                                            success_message = format!("Password added for '{}'", username);
                                                            list_of_passwords = v.list_services().iter().map(|s| s.to_string()).collect();
                                                            username.clear();
                                                            password.clear();
                                                            menu_state = MenuState::SelectMenu;
                                                        }

                                                        Err(e) => {
                                                            error_message = format!("Failed to save: {}", e);
                                                        }
                                                    }
                                                }

                                                Err(e) => {
                                                    error_message = format!("Failed to add: {}", e);
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            AddOrUpdate::Update => {
                                if widgets::Button::new("Update").ui(ui) {
                                    error_message.clear();

                                    if username.is_empty() || password.is_empty() {
                                        error_message = "Service name and password cannot be empty".to_string();
                                    } else {
                                        if let Some(ref mut v) = vault {
                                            match v.update_password(&username, &password) {
                                                Ok(_) => {
                                                    match v.save(&master_password) {
                                                        Ok(_) => {
                                                            // Update the list of passwords
                                                            success_message = format!("Password updated for '{}'", username);
                                                            list_of_passwords = v.list_services().iter().map(|s| s.to_string()).collect();
                                                            username.clear();
                                                            password.clear();
                                                            menu_state = MenuState::SelectMenu;
                                                        }

                                                        Err(e) => {
                                                            error_message = format!("Failed to save: {}", e);
                                                        }
                                                    }
                                                }

                                                Err(e) => {
                                                    error_message = format!("Failed to update: {}", e);
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                        }

                        if widgets::Button::new("Cancel").ui(ui) {
                            error_message.clear();
                            username.clear();
                            password.clear();
                            menu_state = MenuState::SelectMenu;
                        }
                    }
                });
            }
        }

        next_frame().await;
    }
}
