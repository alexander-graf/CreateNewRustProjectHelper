use druid::widget::{Checkbox, Controller, Flex, Padding, TextBox};
use druid::{AppLauncher, Data, Env, Event, EventCtx, Lens, LocalizedString, Widget, WidgetExt, WindowDesc};
use druid::widget::Button;
use std::path::PathBuf;
use dirs;
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
                self.prev_data = data.clone();
            }
            _ => (),
        }
        child.event(ctx, event, data, env);
    }
}

fn build_ui() -> impl Widget<CheckboxData> {
    let text_field = TextBox::new()
        .with_placeholder("put in new name")
        .lens(CheckboxData::new_project_name);

    Flex::column()
        .with_child(Padding::new(10.0, Checkbox::new("Checkbox 1").lens(CheckboxData::checkbox1)))
        .with_child(Padding::new(10.0, Checkbox::new("Checkbox 2").lens(CheckboxData::checkbox2)))
        .with_child(Padding::new(10.0, Checkbox::new("Checkbox 3").lens(CheckboxData::checkbox3)))
        .with_child(Padding::new(10.0, Button::new("Barney").on_click(|_ctx, data: &mut CheckboxData, _env| {
            println!("Button gedrückt");
            println!("{}", data.new_project_name);
            create_new_project(data);
        })))
        .with_child(Padding::new(10.0, text_field))
        .controller(CheckboxController::new())
}

fn main() {
    let main_window = WindowDesc::new(build_ui())
        .title(LocalizedString::new("Checkbox Example"));

    let data = CheckboxData {
        checkbox1: false,
        checkbox2: false,
        checkbox3: false,
        new_project_name: String::new(),
    };

    let button = Button::new("Create Project").on_click(|_ctx, data: &mut CheckboxData, _env| {
        if data.new_project_name.trim().is_empty() {
            println!("The text field is empty.");
            return;
        }

        let rust_projects_path = dirs::home_dir().unwrap().join("rustprojects");
        let project_name = data.new_project_name.clone();

        let output = Command::new("cargo")
                     .args(&["new", &project_name])
                     .current_dir(&rust_projects_path)
                     .output()
                     .expect("Failed to execute command");
         
                 if output.status.success() {
                     println!("Successfully created new project: {}", project_name);
                     let main_rs_path = rust_projects_path.join(&project_name).join("src/main.rs");
                     Command::new("code")
                         .arg(main_rs_path)
                         .spawn()
                         .expect("Failed to open file in VS Code");
                 } else {
                     let error_message = String::from_utf8_lossy(&output.stderr);
                     println!("Failed to create new project: {}", error_message);
                 }
             });
         
             let layout = Flex::column()
                 .with_child(Padding::new(10.0, Checkbox::new("Checkbox 1").lens(CheckboxData::checkbox1)))
                 .with_child(Padding::new(10.0, Checkbox::new("Checkbox 2").lens(CheckboxData::checkbox2)))
                 .with_child(Padding::new(10.0, Checkbox::new("Checkbox 3").lens(CheckboxData::checkbox3)))
                 .with_child(Padding::new(10.0, TextBox::new().with_placeholder("put in new name").lens(CheckboxData::new_project_name)))
                 .with_child(Padding::new(10.0, button))
                 .controller(CheckboxController::new());
         
             let data = CheckboxData {
                 checkbox1: false,
                 checkbox2: false,
                 checkbox3: false,
                 new_project_name: String::new(),
             };
         
             AppLauncher::with_window(main_window)
                 .launch(data)
                 .expect("Failed to launch application");
         }


         fn create_new_project(data: &mut CheckboxData) {
            if data.new_project_name.trim().is_empty() {
                println!("The text field is empty.");
                return;
            }
        
            let rust_projects_path = dirs::home_dir().unwrap().join("rustprojects");
            let project_name = data.new_project_name.clone();
        
            let output = Command::new("cargo")
                         .args(&["new", &project_name])
                         .current_dir(&rust_projects_path)
                         .output()
                         .expect("Failed to execute command");
             
            if output.status.success() {
                println!("Successfully created new project: {}", project_name);
                let main_rs_path = rust_projects_path.join(&project_name).join("src/main.rs");
                Command::new("code")
                    .arg(main_rs_path)
                    .spawn()
                    .expect("Failed to open file in VS Code");
            } else {
                let error_message = String::from_utf8_lossy(&output.stderr);
                println!("Failed to create new project: {}", error_message);
            }
        }