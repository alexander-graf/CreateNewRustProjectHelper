use dirs;
use druid::widget::{Button, Flex, TextBox, WidgetExt};
use druid::{AppLauncher, Data, Lens, LocalizedString, Widget, WindowDesc};
use std::fs;
use std::path::Path;
use std::process::Command;
use std::thread;

#[derive(Clone, Data, Lens)]
struct ProjectData {
    project_name: String,
}

fn assemble_user_interface() -> impl Widget<ProjectData> {
    let project_directory = dirs::home_dir().unwrap().join("rustprojects");

    Flex::column()
        .with_child(create_name_input_field())
        .with_spacer(8.0)
        .with_child(create_new_project_button())
        .with_spacer(8.0)
        .with_child(create_open_project_directory_button(
            project_directory.as_path(),
        ))
}

fn main() {
    let main_window = WindowDesc::new(assemble_user_interface()).title(LocalizedString::new(
        "Create New Rust Project With Cargo Add",
    ));

    let data = ProjectData {
        project_name: String::new(),
    };

    AppLauncher::with_window(main_window)
        .launch(data)
        .expect("Failed to launch application");
}

fn open_project_files(project_directory: &Path) {
    let toml_file_path = project_directory.join("Cargo.toml");
    let main_file_path = project_directory.join("src/main.rs");

    Command::new("code")
        .args(&[
            toml_file_path.to_str().unwrap(),
            main_file_path.to_str().unwrap(),
        ])
        .spawn()
        .expect("Failed to open files in VS Code");
}

fn open_directory_in_thunar(directory_path: &Path) {
    Command::new("thunar")
        .arg(directory_path.to_str().unwrap())
        .spawn()
        .expect("Failed to open folder");
}

fn create_name_input_field() -> impl Widget<ProjectData> {
    TextBox::new()
        .padding(10.0)
        .expand_width()
        .lens(ProjectData::project_name)
}

fn create_new_project_button() -> impl Widget<ProjectData> {
    Button::new("Create Project")
        .padding(10.0)
        .on_click(move |_ctx, data: &mut ProjectData, _env| {
            if data.project_name.trim().is_empty() {
                println!("The text field is empty.");
                return;
            }

            let rust_projects_path = dirs::home_dir().unwrap().join("rustprojects");
            let project_name = data.project_name.clone();
            let new_project_path = rust_projects_path.join(&project_name);

            thread::spawn(move || {
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
                        let cargo_add_values = vec!["add", "druid", "env", "dev"];
                        let output = Command::new("cargo")
                            .args(cargo_add_values)
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
                                    open_project_files(&new_project_path);
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
                    open_project_files(&new_project_path);
                }
            });
        })
}

fn create_open_project_directory_button(main_path: &Path) -> impl Widget<ProjectData> {
    let main_path = main_path.to_path_buf(); // Convert to PathBuf for ownership
    Button::new("Open Folder")
        .padding(10.0)
        .on_click(move |_ctx, data: &mut ProjectData, _env| {
            let path = main_path.join(&data.project_name);
            if path.is_dir() {
                // Check if the directory is empty
                match fs::read_dir(&path) {
                    Ok(mut dir) => {
                        if dir.next().is_none() {
                            // If the directory is empty, open the parent directory
                            if let Some(parent_path) = path.parent() {
                                open_directory_in_thunar(&parent_path);
                            } else {
                                println!("The directory '{}' has no parent.", data.project_name);
                            }
                        } else {
                            // If the directory is not empty, open it
                            open_directory_in_thunar(&path);
                        }
                    }
                    Err(e) => {
                        println!("Failed to read directory '{}': {}", data.project_name, e);
                    }
                }
            } else {
                println!("'{}' is not a directory.", data.project_name);
            }
        })
}
