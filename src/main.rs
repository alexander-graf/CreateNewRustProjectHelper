use std::path::Path;
use dirs;
use druid::widget::{Button, Checkbox, Controller, Flex, Padding, TextBox};
use druid::{
    AppLauncher, Data, Env, Event, EventCtx, Lens, LocalizedString, Widget, WidgetExt, WindowDesc,
};
use std::process::Command;

#[derive(Clone, Data, Lens)]
struct CheckboxData {
    checkbox1: bool,
    checkbox2: bool,
    checkbox3: bool,
    new_project_name: String,
}

struct CheckboxController {
    prev_data: CheckboxData,
}

impl CheckboxController {
    fn new() -> Self {
        Self {
            prev_data: CheckboxData {
                checkbox1: false,
                checkbox2: false,
                checkbox3: false,
                new_project_name: String::new(),
            },
        }
    }
}

impl<W: Widget<CheckboxData>> Controller<CheckboxData, W> for CheckboxController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut CheckboxData,
        env: &Env,
    ) {
        match event {
            Event::MouseUp(_) => {
                if self.prev_data.checkbox1 != data.checkbox1 && data.checkbox1 {
                    println!("Checkbox 1 was checked");
                }
                if self.prev_data.checkbox2 != data.checkbox2 && data.checkbox2 {
                    println!("Checkbox 2 was checked");
                }
                if self.prev_data.checkbox3 != data.checkbox3 && data.checkbox3 {
                    println!("Checkbox 3 was checked");
                }
                self.prev_data.checkbox1 = data.checkbox1;
                self.prev_data.checkbox2 = data.checkbox2;
                self.prev_data.checkbox3 = data.checkbox3;
            }
            _ => (),
        }
        child.event(ctx, event, data, env);
    }
}

fn build_ui() -> impl Widget<CheckboxData> {
    let text_field = TextBox::new()
        .with_placeholder("put in newwwwww name")
        .lens(CheckboxData::new_project_name);

    Flex::column()
        .with_child(Padding::new(
            10.0,
            Checkbox::new("Checkbox 1").lens(CheckboxData::checkbox1),
        ))
        .with_child(Padding::new(
            10.0,
            Checkbox::new("Checkbox 2").lens(CheckboxData::checkbox2),
        ))
        .with_child(Padding::new(
            10.0,
            Checkbox::new("Checkbox 3").lens(CheckboxData::checkbox3),
        ))
        .with_child(Padding::new(
            10.0,
            Button::new("Create New Project").on_click(|_ctx, data: &mut CheckboxData, _env| {
                println!("Button gedrückt");
                println!("{}", data.new_project_name);
                create_new_project(&data.new_project_name);
            }),
        ))
        .with_child(Padding::new(10.0, text_field))
        .controller(CheckboxController::new())
}

fn main() {
    let main_window = WindowDesc::new(build_ui()).title(LocalizedString::new(
        "Create New Rust Project With Cargo Add",
    ));

    let data = CheckboxData {
        checkbox1: false,
        checkbox2: false,
        checkbox3: false,
        new_project_name: String::new(),
    };

    let _button =
        Button::new("Create Project").on_click(move |_ctx, data: &mut CheckboxData, _env| {
            if data.new_project_name.trim().is_empty() {
                println!("The komische text field is empty.");
                return;
            }

            let rust_projects_path = dirs::home_dir().unwrap().join("rustprojects");
            let project_name = data.new_project_name.clone();

            println!("Now ruuuuun");

            // 1. Run `cargo new`
            let output = Command::new("cargo")
                .args(&["new", &project_name])
                .current_dir(&rust_projects_path)
                .output()
                .expect("Failed to execute command");

            if output.status.success() {
                println!("Successfully created newwww project: {}", project_name);

                let new_project_path = rust_projects_path.join(&project_name);

                println!("About to ruuuuun 'cargo add druid' in the new project's directory");

                let output = Command::new("cargo")
                    .args(&["add", "druid"])
                    .current_dir(&new_project_path)
                    .output();

                match output {
                    Ok(output) => {
                        if output.status.success() {
                            println!(
                                "Successfully added druid deppppendency to the project: {}",
                                project_name
                            );
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
                // 2. Run `cargo add druid` in the new project's directory
                let output = Command::new("cargo")
                    .args(&["add", "druid"])
                    .current_dir(&new_project_path)
                    .output()
                    .expect("Failed to execute command");

                if output.status.success() {
                    println!(
                        "Successfully added druid dependency to the project: {}",
                        project_name
                    );

                    // 3. Open the new project in VS Code
                    Command::new("code")
                        .arg(new_project_path)
                        .spawn()
                        .expect("Failed to open project in VS Code");
                } else {
                    let error_message = String::from_utf8_lossy(&output.stderr);
                    println!("Failed to add druid dependency: {}", error_message);
                }
            } else {
                let error_message = String::from_utf8_lossy(&output.stderr);
                println!("Failed to create new project: {}", error_message);
            }
        });

    AppLauncher::with_window(main_window)
        .launch(data)
        .expect("Failed to launch application");
}

fn create_new_project(project_name: &str) {
    // check if empty
    if project_name.trim().is_empty() {
        println!("The text field is empty.");
        return;
    }

    let rust_projects_path = dirs::home_dir().unwrap().join("rustprojects");
    //let project_name = project_name.clone();

    let output = Command::new("cargo")
        .args(&["new", &project_name])
        .current_dir(&rust_projects_path)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("Successfully created new project: {}", project_name);

        let new_project_path = rust_projects_path.join(&project_name);

        println!("About to run 'cargo add druid' in the new project's directory");

        let output = Command::new("cargo")
            .args(&["add", "druid"])
            .current_dir(&new_project_path)
            .output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    println!(
                        "Successfully added druid dependency to the projjjject: {}",
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
    }
}


fn open_new_toml_and_new_main(new_project_path: &Path) {
    let toml_path = new_project_path.join("Cargo.toml");
    let main_rs_path = new_project_path.join("src/main.rs");

    Command::new("code")
        .args(&[toml_path.to_str().unwrap(), main_rs_path.to_str().unwrap()])
        .spawn()
        .expect("Failed to open files in VS Code");
}
