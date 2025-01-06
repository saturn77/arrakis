mod components;
mod models;
mod state; 
mod parameters;


pub use state::{Tab, ApplicationState};
pub use components::logger::Logger;
pub use components::project::{Project, project_tab_ui};
pub use components::settings::settings_tab_ui;

pub use models::details::Details;
pub use models::initial::Banner; 


use std::sync::{Arc, Mutex};
use egui_aesthetix::{
    themes::{
        CarlDark, NordDark, NordLight, StandardDark, StandardLight, TokyoNight, TokyoNightStorm,
    },
    Aesthetix,
};
use eframe::egui; 

use std::rc::Rc;
use std::collections::BTreeMap;




pub struct MyApp {
    pub logger_text: Logger,
    // wrap the scroller_text in an Arc to allow for multiple references
    pub scroller_text: Arc<Mutex<String>>,
    pub cursor_update : bool,
    pub buffer_text : Arc<Mutex<String>>,
    themes: Vec<Rc<dyn Aesthetix>>,
    project : Project,
    state : ApplicationState,
    /// Tab labels and icons
    tab_labels: BTreeMap<Tab, &'static str>,
}

impl MyApp { 

    #[must_use]
    pub fn new(creation_context: &eframe::CreationContext<'_>) -> Self {

        let themes: Vec<Rc<dyn Aesthetix>> = vec![
            Rc::new(TokyoNightStorm),
            Rc::new(TokyoNight),
            Rc::new(StandardDark),
            Rc::new(StandardLight),
            Rc::new(CarlDark),
            Rc::new(NordDark),
            Rc::new(NordLight),
            
            
        ];

        let active_theme : Rc<dyn Aesthetix> = match themes.first() {
            Some(theme) => Rc::clone(theme),
            None => Rc::new(TokyoNightStorm),
        };

        // Initialize the custom theme/styles for egui
        creation_context
            .egui_ctx
            .set_style(active_theme.custom_style());

        let mut initial_display : Banner = Banner::new(); 
        initial_display.format();
        
        let yy : Arc<Mutex<String>> = Arc::new(Mutex::new(initial_display.message.clone()));
        let zz = yy.clone();

        Self {
            logger_text: Logger::default(),
            scroller_text: zz,
            buffer_text : Arc::new(Mutex::new(String::new())),
            cursor_update : false,
            themes,
            project : Project::new(),
            state   : ApplicationState::new(active_theme),
            tab_labels: [
                (Tab::Home, "🏠  Home"),
                (Tab::Project, "⚙  Project"),
                (Tab::Build, "📝  Build"),
                (Tab::About, "ℹ  About"),
            ]
            .into_iter()
            .collect(),
        }
    }




}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // This builds the main side navigation panel
        egui::SidePanel::left("main_side_panel")
            .resizable(false)
            .frame(
                egui::Frame::none()
                    .fill(self.state.active_theme.bg_secondary_color_visuals())
                    .inner_margin(self.state.active_theme.margin_style())
                    .stroke(egui::Stroke::new(
                        1.0,
                        self.state.active_theme.bg_secondary_color_visuals(),
                    )),
            )
            .exact_width(200.0)
            .show(ctx, |ui_side_panel| {
                ui_side_panel.with_layout(
                    egui::Layout::top_down_justified(egui::Align::Center),
                    |ui_layout| {
                        ui_layout.add_space(0.0);
                        ui_layout.heading(egui::RichText::new(parameters::gui::DASHBOARD_TITLE).size(20.0).strong());
                        //annoying 
                        // egui::warn_if_debug_build(ui_layout);
                    },
                );

                ui_side_panel.with_layout(
                    egui::Layout::top_down_justified(egui::Align::Min),
                    |ui_layout| {
                        for (tab, label) in &self.tab_labels {
                            ui_layout.selectable_value(&mut self.state.active_tab, *tab, *label);
                        }
                    },
                );




                settings_tab_ui(ui_side_panel, &mut self.state, &self.themes);

                
                    
                
            });

        egui::CentralPanel::default().show(ctx, |ui| {

            
            ui.separator();

            // add the project tab

            // Display the image of the project if the home tab is active
            if self.state.active_tab == Tab::Home {
                ui.with_layout(
                    egui::Layout::top_down_justified(egui::Align::TOP),
                    |ui_layout| {
                        // add the image, but also center it
                        ui_layout.add_space(15.0);
                        ui_layout.add(
                        egui::Image::new(egui::include_image!("../assets/hyperion.png"))
                            .rounding(10.0)
                            .max_size(egui::Vec2 { x: 500.0, y: 400.0 }));

                    });
            }

            // Display the project_tab_ui if the active tab is the project tab
            if self.state.active_tab == Tab::Project {
                project_tab_ui(ctx, ui, &mut self.project);
            }

            // project_tab_ui(ctx, ui, &mut self.project);

            // // add the settings tab
            

            // ============================================================
            // ** TERMINAL ** Display messages from GUI actions
            // ============================================================
            //let logger_frame = egui::Frame::default().rounding(5.0);
            // .fill(self.state.active_theme.bg_secondary_color_visuals())
            // .shadow(Shadow::default());


            ui.vertical(|ui| {
                ui.heading("Terminal");
                ui.separator();
            

            
            // egui::Window::new("Logger Terminal")
            //     .frame(logger_frame)
            //     .show(ctx, |ui| {
            //         ui.add_space( 0.0);
                    ui.horizontal(|ui| {
                        if ui
                            .button(egui::RichText::new("Clear").color(egui::Color32::GREEN))
                            .clicked()
                        {
                            *self.scroller_text.lock().unwrap() = self.logger_text.clear();
                        }

                        if ui
                            .button(egui::RichText::new("System Info").color(egui::Color32::GREEN))
                            .clicked()
                        {
                            *self.scroller_text.lock().unwrap() = self.logger_text.system_info();
                        }
                        if ui
                            .button(egui::RichText::new("FPGA Version").color(egui::Color32::GREEN))
                            .clicked()
                        {
                            *self.scroller_text.lock().unwrap() = self.logger_text.get_fpga_version();
                            // subprocess command to get the exa tree output
                            let output = std::process::Command::new("exa")
                                .arg("--tree")
                                .arg("--level=3")
                                .output()
                                .expect("failed to execute process");
                            *self.scroller_text.lock().unwrap() = String::from_utf8_lossy(&output.stdout).into();
                        }




                        if ui
                        .button(egui::RichText::new("Get Serial").color(egui::Color32::GREEN))
                        .clicked(){
                                    // create a csv reader object
                                // let source_path_buffer = std::path::PathBuf::from("/home/james/raid_one/software_projects/atlantix/Egui/test_image/src/models/fpga.csv");
                                // let source_file_handle = std::fs::File::open(source_path_buffer.clone()).unwrap();
                                // let mut reader_obj = csv::Reader::from_reader(source_file_handle);
                
                                // // create a CsvFile object, get the headers and print them to the console
                                // let mut csv_file = CsvFile::new();
                                // csv_file.get_headers(&mut reader_obj).unwrap();
                                // csv_file.get_column_vectors(&mut reader_obj).unwrap();
                                // *self.scroller_text.lock().unwrap() = csv_file.format_headers_with_column_count();  
                            }
                        

                    }); // end horizontal

                    let alpha: &mut bool = &mut false;
                    // ============================================================
                    // ** TERMINAL::TEXT_EDIT **
                    // ============================================================



                    let mut _scroller =
                        egui::ScrollArea::vertical()
                            .id_source("scrollerx")
                            .stick_to_bottom(*alpha)
                            .show(ui, |ui| {


                                let output = 
                                egui::TextEdit::multiline(
                                &mut *self.scroller_text.lock().unwrap(),
                                
                            )
                            .id(egui::Id::new("terminal"))
                            .text_color(egui::Color32::GREEN)
                            .font(egui::TextStyle::Monospace) // for cursor height
                            .interactive(true)
                            .desired_rows(20)
                            .lock_focus(true)
                            .desired_width(550.)
                            .show(ui);

                                // if terminal text is changed and the Enter key was pressed, echo scroller text to stdout
                                if ui.input(|i| i.key_pressed(egui::Key::Enter)) {

                                    let mut scroller_bytes = self.scroller_text.lock().unwrap().as_bytes().to_vec();
                                    let mut j  = scroller_bytes.len();
                                    let mut scroller_text = String::new();

                                    while j > 0 {
                                        match scroller_bytes[j-1] {

                                            10 => {

                                                
                                                scroller_bytes.truncate(scroller_bytes.len() - 1);
                                            },
                                            _ => {
                                                println!("\n Buffer processed ...");
                                                // now print the scroller_bytes as a String 
                                                println!("Truncated to length: {}", scroller_bytes.len());
                                                scroller_text = String::from_utf8(scroller_bytes.clone()).unwrap();
                                                println!("Scroller Text = {}", scroller_text);
                                                break; 
                                            }
                                        }
                                        j = j - 1; 
                                    }             
                                    *self.buffer_text.lock().unwrap() = scroller_text.clone(); 
                                    println!("Buffer Text = {}", self.buffer_text.lock().unwrap());  
                                    // check to see if the last characters of scroller_text are equal to "clear"
                                    // using a regex to match the last 5 characters of the scroller_text
                                    // if the last 5 characters are "clear", then clear the scroller_text
                                    // if the last 5 characters are "exit", then exit the program

                                    let re_clear = regex::Regex::new(r"clear").unwrap();
                                    //let re_exit = regex::Regex::new(r"exit").unwrap();
                                    let re_version = regex::Regex::new(r"version").unwrap();
                                    let re_help = regex::Regex::new(r"help").unwrap();
                                    let re_system = regex::Regex::new(r"system").unwrap();


                                    if re_clear.is_match(&scroller_text) {
                                        *self.scroller_text.lock().unwrap() = self.logger_text.clear();
                                        self.cursor_update = true;
                                    }
                                
                                    else if re_system.is_match(&scroller_text) {
                                        *self.scroller_text.lock().unwrap() = self.logger_text.system_info();
                                        self.cursor_update = true;
                                    }
                                    
                                    else if re_version.is_match(&scroller_text) {
                                        // clear the logger first 
                                        *self.scroller_text.lock().unwrap() = self.logger_text.clear();
                                        let mut initial_display = Banner::new();
                                        initial_display.format(); 
                                        *self.scroller_text.lock().unwrap() += & initial_display.message;
                                        self.cursor_update = true;

                                    }

                                    else if re_help.is_match(&scroller_text) {
                                        *self.scroller_text.lock().unwrap() =  "Commands:\n".to_string();
                                        *self.scroller_text.lock().unwrap() += "clear   - clear the terminal";
                                        *self.scroller_text.lock().unwrap() += "\nversion - print the version information for Vescript";
                                        *self.scroller_text.lock().unwrap() += "\nsystem  - print the OS system info for host machine\n";
                                        *self.scroller_text.lock().unwrap() += "exit \n";
                                        self.cursor_update = true;
                                    }

                                    else {

                                        }            
                                } // end if ui.input

                                
                            if self.cursor_update {
                                if let Some(mut state) = egui::TextEdit::load_state(ui.ctx(), output.response.id) {
                                    let ccursor = egui::text::CCursor::new(self.scroller_text.lock().unwrap().chars().count());
                                    state
                                        .cursor
                                        .set_char_range(Some(egui::text::CCursorRange::one(ccursor)));
                                    state.store(ui.ctx(), output.response.id);
                                    ui.ctx().memory_mut(|mem| mem.request_focus(output.response.id)); // give focus back to the [`TextEdit`].
                                }
                                self.cursor_update = false;
                            }


                                
                                

                            }); // end scroll area
                            


                }); // end vertical

            

        });
        

    }
}



// mod app

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).




    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([
            parameters::gui::VIEWPORT_X, 
            parameters::gui::VIEWPORT_Y
        ]),
        ..Default::default()
    };

    eframe::run_native(
        parameters::gui::WINDOW_TITLE,
        options,
        //egui_extras::install_image_loaders(&cc.egui_ctx), 
        // Box::new(move |cc| Box::new({
        //     // This gives us image support:
        //     egui_extras::install_image_loaders(&cc.egui_ctx);
        //     MyApp::new(cc) 
        // })),
        Box::new(move |cc| Ok(Box::new({
            egui_extras::install_image_loaders(&cc.egui_ctx);
            MyApp::new(cc)}))),
    )
}
// #[cfg(target_arch = "wasm32")]
// fn main() {
//     // Redirect `log` message to `console.log` and friends:
//     eframe::WebLogger::init(log::LevelFilter::Debug).ok();

//     let web_options = eframe::WebOptions::default();

//     wasm_bindgen_futures::spawn_local(async {
//         eframe::WebRunner::new()
//             .start(
//                 "the_canvas_id", // hardcode it
//                 web_options,
//                 Box::new(|cc| Box::new({
//                     egui_extras::install_image_loaders(&cc.egui_ctx);
//                     MyApp::new(cc)
//                     })
//                 ),
                
//             )
//             .await
//             .expect("failed to start eframe");
//     });
// }
