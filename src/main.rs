use dirs;
use druid::widget::{Button, Flex, TextBox, WidgetExt};
use druid::{AppLauncher, Data, Lens, LocalizedString, Widget, WindowDesc};
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Clone, Data, Lens)]
struct ProjectData {
    new_project_name: String,
}

fn build_ui() -> impl Widget<ProjectData> {
    let main_path = dirs::home_dir().unwrap().join("rustprojects");

    Flex::column()
        .with_child(create_text_field())
        .with_spacer(8.0) // Adds some space between the text field and the button
        .with_child(create_project_button())
        .with_spacer(8.0) // Adds some space between the "Create Project" and "Open Folder" buttons
        .with_child(create_open_folder_button(main_path.as_path()))
}

fn main() {
    let main_window = WindowDesc::new(build_ui()).title(LocalizedString::new(
        "Create New Rust Project With Cargo Add",
    ));

    let data = ProjectData {
        new_project_name: String::new(),
    };

    AppLauncher::with_window(main_window)
        .launch(data)
        .expect("Failed to launch application");
}

fn open_new_toml_and_new_main(new_project_path: &Path) {
    let toml_path = new_project_path.join("Cargo.toml");
    let main_rs_path = new_project_path.join("src/main.rs");

    Command::new("code")
        .args(&[toml_path.to_str().unwrap(), main_rs_path.to_str().unwrap()])
        .spawn()
        .expect("Failed to open files in VS Code");
}

fn open_folder(path: &Path) {
    Command::new("thunar")
        .arg(path.to_str().unwrap())
        .spawn()
        .expect("Failed to open folder");
}

fn create_text_field() -> impl Widget<ProjectData> {
    TextBox::new()
        .padding(10.0)
        .expand_width()
        .lens(ProjectData::new_project_name)
}

fn create_project_button() -> impl Widget<ProjectData> {
    Button::new("Create Project")
        .padding(10.0)
        .on_click(move |_ctx, data: &mut ProjectData, _env| {
            if data.new_project_name.trim().is_empty() {
                println!("The text field is empty.");
                return;
            }

            let rust_projects_path = dirs::home_dir().unwrap().join("rustprojects");
            let project_name = data.new_project_name.clone();
            let new_project_path = rust_projects_path.join(&project_name);

            if !new_project_path.exists() {
                println!("Now running 'cargo new'");

                let output = Command::new("cargo")
                    .args(&["new", &project_name])
                    .current_dir(&rust_projects_path)
                    .output()
                    .expect("Failed to execute command");

                if output.status.success() {
                    println!("Successfully created new project: {}", project_name);

                    println!("About to run 'cargo add druid' in the new project's directory");

                    let output = Command::new("cargo")
                        .args(&["add", "druid"])
                        .current_dir(&new_project_path)
                        .output();

                    match output {
                        Ok(output) => {
                            if output.status.success() {
                                println!(
                                    "Successfully added druid dependency to the project: {}",
                                    project_name
                                );
                                println!("Opening new project in VS Code");
                                println!("{}", new_project_path.display());
                                open_new_toml_and_new_main(&new_project_path);
                            } else {
                                println!(
                                    "Failed to add druid dependency. Output was: \n{}\nError was: \n{}",
                                    String::from_utf8_lossy(&output.stdout),
                                    String::from_utf8_lossy(&output.stderr)
                                );
                            }
                        }
                        Err(e) => {
                            println!("Failed to execute command: {}", e);
                        }
                    }
                } else {
                    let error_message = String::from_utf8_lossy(&output.stderr);
                    println!("Failed to create new project: {}", error_message);
                }
            } else {
                println!("Project already exists. Opening files in VS Code.");
                open_new_toml_and_new_main(&new_project_path);
            }
        })
}

fn create_open_folder_button(main_path: &Path) -> impl Widget<ProjectData> {
    let main_path = main_path.to_path_buf(); // Convert to PathBuf for ownership
    Button::new("Open Folder")
        .padding(10.0)
        .on_click(move |_ctx, data: &mut ProjectData, _env| {
            let path = main_path.join(&data.new_project_name);
            if path.is_dir() {
                // Check if the directory is empty
                match fs::read_dir(&path) {
                    Ok(mut dir) => {
                        if dir.next().is_none() {
                            // If the directory is empty, open the parent directory
                            if let Some(parent_path) = path.parent() {
                                open_folder(&parent_path);
                            } else {
                                println!(
                                    "The directory '{}' has no parent.",
                                    data.new_project_name
                                );
                            }
                        } else {
                            // If the directory is not empty, open it
                            open_folder(&path);
                        }
                    }
                    Err(e) => {
                        println!(
                            "Failed to read directory '{}': {}",
                            data.new_project_name, e
                        );
                    }
                }
            } else {
                println!("'{}' is not a directory.", data.new_project_name);
            }
        })
}
