

use egui_file_dialog::FileDialog;

#[derive(Default)]
pub struct Project {
    pub id: i32,
    pub file_dialog: FileDialog,
    pub project_name: String,
    pub project_dir: String,
    pub module_name: String,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
    pub repo_url: String,
    pub company: String,
}

impl Project {
    pub fn new() -> Project {
        Project {
            id: 0,
            file_dialog  : FileDialog::new(),
            project_name : String::new(),
            project_dir  : String::new(),
            module_name  : String::new(),
            description  : String::new(),
            created_at   : String::new(),
            updated_at   : String::new(),
            repo_url     : String::new(),
            company      : String::new(),
        }
    }
}

pub const TEXT_SIZE: f32 = 12.0;
pub const TEXT_EDIT_WIDTH: f32 = 400.0;


pub fn project_tab_ui(
    ctx: &egui::Context,
    ui_root: &mut egui::Ui,
    project: &mut Project,
) {
    egui::ScrollArea::new([false, true])
        .id_source("settings_tab_scroll_area")
        .max_width(800.)
        .stick_to_bottom(false)
        .show(ui_root, |ui_scroll_area| {
            ui_scroll_area.add_space(20.0);

            ui_scroll_area.with_layout(
                egui::Layout::top_down_justified(egui::Align::Center),
                |ui_layout| {
                    egui::Grid::new("settings_grid")
                    .striped(true)
                    .num_columns(2)
                    .striped(true)
                    .show(ui_layout, |ui_grid| {
                        ui_grid.add(egui::Label::new(
                            egui::RichText::new("Project Name")
                                .size(TEXT_SIZE)
                                .monospace(),
                        ));
                        ui_grid.add(egui::TextEdit::singleline(&mut project.project_name)
                        .hint_text("Set name of Logging Project i.e. RealData_01Jan2026.csv")
                        .desired_width(TEXT_EDIT_WIDTH));
                        ui_grid.end_row();

                        ui_grid.add(egui::Label::new(
                            egui::RichText::new("Project Directory")
                                .size(TEXT_SIZE)
                                .monospace(),
                        ));
                        if ui_grid
                            .add(
                                egui::TextEdit::singleline(&mut project.project_dir)
                                    .hint_text("Select a directory to save the project")
                                    .desired_width(TEXT_EDIT_WIDTH),
                            )
                            .double_clicked_by(egui::PointerButton::Primary)
                        {
                            project.file_dialog.select_directory();
                        }

                        if let Some(path) = project.file_dialog.update(ctx).selected() {
                            project.project_dir =
                                path.to_path_buf().to_string_lossy().to_string();
                        }
                        ui_grid.end_row();

                        ui_grid.add(egui::Label::new(
                            egui::RichText::new("Module Name")
                                .size(TEXT_SIZE)
                                .monospace(),
                        ));

                        ui_grid.add(
                            egui::TextEdit::singleline(&mut project.module_name)
                                .desired_width(TEXT_EDIT_WIDTH),
                        );

                        ui_grid.end_row();

                        ui_grid.add(egui::Label::new(
                            egui::RichText::new("Description")
                                .size(TEXT_SIZE)
                                .monospace(),
                        ));

                        ui_grid.add(egui::TextEdit::multiline(&mut project.description));

                        ui_grid.end_row();

                        ui_grid.add(egui::Label::new(
                            egui::RichText::new("Created At")
                                .size(TEXT_SIZE)
                                .monospace(),
                        ));

                        ui_grid.add(egui::TextEdit::singleline(&mut project.created_at));

                        ui_grid.end_row();

                        ui_grid.add(egui::Label::new(
                            egui::RichText::new("Updated At")
                                .size(TEXT_SIZE)
                                .monospace(),
                        ));

                        ui_grid.add(egui::TextEdit::singleline(&mut project.updated_at));

                        ui_grid.end_row();

                        ui_grid.add(egui::Label::new(
                            egui::RichText::new("Repo Url").size(TEXT_SIZE).monospace(),
                        ));

                        ui_grid.add(egui::TextEdit::singleline(&mut project.repo_url));

                        ui_grid.end_row();

                        ui_grid.add(egui::Label::new(
                            egui::RichText::new("Company").size(TEXT_SIZE).monospace(),
                        ));

                        ui_grid.add(egui::TextEdit::singleline(&mut project.company));

                        ui_grid.end_row();
                    });
                },
            );
        });
}
