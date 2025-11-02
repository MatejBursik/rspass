use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};
use clipboard::{ClipboardContext, ClipboardProvider};

mod encrypt;
mod vault;

use vault::Vault;

#[derive(PartialEq)]
enum MenuState {
    LogInMenu,
    InitScreen,
    SelectMenu,
    AddUpdateScreen,
    RemoveConfirmation
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

#[derive(PartialEq)]
enum ClearOrShow {
    Clear,
    Show
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
    let mut clear_or_show = ClearOrShow::Clear;

    let mut clipboard_ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let mut vault: Option<Vault> = None;
    let mut master_password = String::new();
    let mut username = String::new();
    let mut password = String::new();
    let mut combobox = 0;
    let mut list_of_passwords: Vec<String> = vec!["None".to_string()];
    let mut service_to_remove = String::new();
    let mut message = String::new();

    loop {
        clear_background(GRAY);

        match menu_state {
            MenuState::LogInMenu => {
                root_ui().window(hash!(), vec2(50.0, 50.0), vec2(500.0, 500.0), |ui| {
                    ui.label(None, "Master Password:");
                    widgets::InputText::new(hash!()).password(true).size(vec2(260.0, 30.0)).ui(ui, &mut master_password);

                    if !message.is_empty() {
                        ui.label(None, &format!("{}", message));
                    }

                    if widgets::Button::new("Create Vault").ui(ui) {
                        message.clear();
                        master_password.clear();
                        menu_state = MenuState::InitScreen;
                    }

                    if widgets::Button::new("Log In").ui(ui) {
                        message.clear();

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
                                    clear_or_show = ClearOrShow::Clear;
                                }

                                Err(e) => {
                                    message = format!("Error: Failed to load vault ({})", e);
                                }
                            }
                        } else {
                            message = "Error: Master password cannot be empty".to_string();
                        }
                    }    
                });
            }

            MenuState::InitScreen => {
                root_ui().window(hash!(), vec2(50.0, 50.0), vec2(500.0, 500.0), |ui| {
                    ui.label(None, "Create New Vault");
                    ui.label(None, "Master Password:");
                    widgets::InputText::new(hash!()).password(true).size(vec2(260.0, 30.0)).ui(ui, &mut master_password);

                    if !message.is_empty() {
                        ui.label(None, &format!("{}", message));
                    }

                    if widgets::Button::new("Create").ui(ui) {
                        message.clear();

                        if !master_password.is_empty() {
                            match Vault::create_new(&master_password) {
                                Ok(_) => {
                                    menu_state = MenuState::LogInMenu;
                                    master_password.clear();
                                }

                                Err(e) => {
                                    message = format!("Error: Failed to create vault ({})", e);
                                }
                            }
                        } else {
                            message = "Error: Master password cannot be empty".to_string();
                        }
                    }

                    if widgets::Button::new("Cancel").ui(ui) {
                        message.clear();
                        master_password.clear();
                        menu_state = MenuState::LogInMenu;
                    }
                });
            }

            MenuState::SelectMenu => {
                root_ui().window(hash!(), vec2(50.0, 50.0), vec2(500.0, 500.0), |ui| {
                    if login_state == LogInState::In {
                        if widgets::Button::new("Log Out").ui(ui) {
                            vault = None;
                            login_state = LogInState::Out;
                            menu_state = MenuState::LogInMenu;
                            master_password.clear();
                            username.clear();
                            password.clear();
                            clear_or_show = ClearOrShow::Clear;
                            list_of_passwords = vec!["None".to_string()];
                            combobox = 0;
                            message.clear();
                        }

                        if !message.is_empty() {
                            ui.label(None, &format!("{}", message));
                        }

                        // List of passwords
                        let list_refs: Vec<&str> = list_of_passwords.iter().map(|s| s.as_str()).collect();
                        ui.combo_box(hash!(), ": Passwords in the Vault", &list_refs, &mut combobox);

                        // Username and Password copy buttons
                        if widgets::Button::new(username.as_str()).ui(ui) {
                            clipboard_ctx.set_contents(username.clone()).unwrap();
                            message = format!("Success: Username copied into clipboard");
                        }

                        if widgets::Button::new(password.as_str()).ui(ui) {
                            clipboard_ctx.set_contents(password.clone()).unwrap();
                            message = format!("Success: Password copied into clipboard");
                        }

                        match clear_or_show {
                            ClearOrShow::Clear => {
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

                                    clear_or_show = ClearOrShow::Show;
                                }
                            }

                            ClearOrShow::Show => {
                                if widgets::Button::new("Clear").ui(ui) {
                                    message.clear();
                                    username.clear();
                                    password.clear();
                                    clear_or_show = ClearOrShow::Clear;
                                }
                            }
                        }
                        
                        if widgets::Button::new("Add").ui(ui) {
                            message.clear();
                            add_or_update = AddOrUpdate::Add;
                            username.clear();
                            password.clear();
                            clear_or_show = ClearOrShow::Clear;
                            menu_state = MenuState::AddUpdateScreen;
                        }

                        if widgets::Button::new("Update").ui(ui) {
                            message.clear();

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
                                clear_or_show = ClearOrShow::Clear;
                                menu_state = MenuState::AddUpdateScreen;
                            } else {
                                message = "Error: No passwords to update".to_string();
                            }
                        }

                        if widgets::Button::new("Remove").ui(ui) {
                            message.clear();

                            if !list_of_passwords.is_empty() && list_of_passwords[0] != "None" {
                                service_to_remove = list_of_passwords[combobox].clone();
                                clear_or_show = ClearOrShow::Clear;
                                menu_state = MenuState::RemoveConfirmation;
                            } else {
                                message = "Error: No passwords to delete".to_string();
                            }
                        }
                    }
                });
            }

            MenuState::AddUpdateScreen => {
                root_ui().window(hash!(), vec2(50.0, 50.0), vec2(500.0, 500.0), |ui| {
                    if login_state == LogInState::In {
                        if widgets::Button::new("Log Out").ui(ui) {
                            vault = None;
                            login_state = LogInState::Out;
                            menu_state = MenuState::LogInMenu;
                            master_password.clear();
                            username.clear();
                            password.clear();
                            clear_or_show = ClearOrShow::Clear;
                            list_of_passwords = vec!["None".to_string()];
                            combobox = 0;
                            message.clear();
                        }

                        if !message.is_empty() {
                            ui.label(None, &format!("{}", message));
                        }

                        // Text fields: Username, Password
                        ui.label(None, "Username:");
                        widgets::InputText::new(hash!()).size(vec2(260.0, 30.0)).ui(ui, &mut username);

                        ui.label(None, "Password:");
                        widgets::InputText::new(hash!()).size(vec2(260.0, 30.0)).ui(ui, &mut password);
                        
                        match add_or_update {
                            AddOrUpdate::Add => {
                                if widgets::Button::new("Add").ui(ui) {
                                    message.clear();

                                    if username.is_empty() || password.is_empty() {
                                        message = "Error: Service name and password cannot be empty".to_string();
                                    } else {
                                        if let Some(ref mut v) = vault {
                                            match v.add_password(&username, &password) {
                                                Ok(_) => {
                                                    match v.save(&master_password) {
                                                        Ok(_) => {
                                                            // Update the list of passwords
                                                            message = format!("Success: Password added for '{}'", username);
                                                            list_of_passwords = v.list_services().iter().map(|s| s.to_string()).collect();
                                                            username.clear();
                                                            password.clear();
                                                            clear_or_show = ClearOrShow::Clear;
                                                            menu_state = MenuState::SelectMenu;
                                                        }

                                                        Err(e) => {
                                                            message = format!("Error: Failed to save ({})", e);
                                                        }
                                                    }
                                                }

                                                Err(e) => {
                                                    message = format!("Error: Failed to add ({})", e);
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            AddOrUpdate::Update => {
                                if widgets::Button::new("Update").ui(ui) {
                                    message.clear();

                                    if username.is_empty() || password.is_empty() {
                                        message = "Error: Service name and password cannot be empty".to_string();
                                    } else {
                                        if let Some(ref mut v) = vault {
                                            match v.update_password(&username, &password) {
                                                Ok(_) => {
                                                    match v.save(&master_password) {
                                                        Ok(_) => {
                                                            // Update the list of passwords
                                                            message = format!("Success: Password updated for '{}'", username);
                                                            list_of_passwords = v.list_services().iter().map(|s| s.to_string()).collect();
                                                            username.clear();
                                                            password.clear();
                                                            clear_or_show = ClearOrShow::Clear;
                                                            menu_state = MenuState::SelectMenu;
                                                        }

                                                        Err(e) => {
                                                            message = format!("Error: Failed to save ({})", e);
                                                        }
                                                    }
                                                }

                                                Err(e) => {
                                                    message = format!("Error: Failed to update ({})", e);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        if widgets::Button::new("Cancel").ui(ui) {
                            message.clear();
                            username.clear();
                            password.clear();
                            clear_or_show = ClearOrShow::Clear;
                            menu_state = MenuState::SelectMenu;
                        }
                    }
                });
            }

            MenuState::RemoveConfirmation => {
                root_ui().window(hash!(), vec2(50.0, 50.0), vec2(500.0, 500.0), |ui| {
                    ui.label(None, &format!("Do you want to REMOVE password for '{}'?", service_to_remove));
                    ui.label(None, "This action cannot be undone.");
                    
                    if widgets::Button::new("Confirm").ui(ui) {
                        if let Some(ref mut v) = vault {
                            match v.remove_password(&service_to_remove) {
                                Ok(removed) => {
                                    if removed {
                                        match v.save(&master_password) {
                                            Ok(_) => {
                                                // Update the list of passwords
                                                message = format!("Success: Password removed for '{}'", service_to_remove);
                                                list_of_passwords = v.list_services().iter().map(|s| s.to_string()).collect();

                                                if list_of_passwords.is_empty() {
                                                    list_of_passwords = vec!["None".to_string()];
                                                }

                                                service_to_remove.clear();
                                                combobox = 0;
                                                username.clear();
                                                password.clear();
                                                clear_or_show = ClearOrShow::Clear;
                                                menu_state = MenuState::SelectMenu;
                                            }

                                            Err(e) => {
                                                message = format!("Error: Failed to save ({})", e);
                                            }
                                        }
                                    } else {
                                        message = format!("Error: No password found for '{}'", service_to_remove);
                                    }
                                }

                                Err(e) => {
                                    message = format!("Error: Failed to remove ({})", e);
                                }
                            }
                        }
                    }

                    if widgets::Button::new("Cancel").ui(ui) {
                        service_to_remove.clear();
                        combobox = 0;
                        username.clear();
                        password.clear();
                        clear_or_show = ClearOrShow::Clear;
                        menu_state = MenuState::SelectMenu;
                    }
                });
            }
        }

        next_frame().await;
    }
}
