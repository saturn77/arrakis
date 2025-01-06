//! Settings Tab

use crate::state::ApplicationState;
use eframe::egui;
use egui_aesthetix::Aesthetix;
use std::rc::Rc;


#[allow(dead_code)]
struct SettingsTab {
    active_theme: Rc<dyn Aesthetix>,
} 

#[allow(dead_code)]
impl SettingsTab {
    pub fn new(active_theme: Rc<dyn Aesthetix>) -> Self {
        Self {
            active_theme,
        }
    }
}

/// Renders the settings page
pub fn settings_tab_ui(
    ui_root : &mut egui::Ui,
    state   : &mut ApplicationState,
    themes  : &[Rc<dyn Aesthetix>],
) {
    egui::ScrollArea::new([false, true])
        .id_source("settings_component")
        .show(ui_root, |ui_scroll_area| {
            ui_scroll_area.add_space(20.0);

            ui_scroll_area.with_layout(
                egui::Layout::top_down_justified(egui::Align::Center),
                |ui_layout| {
                    egui::Grid::new("settings_grid")
                        .striped(true)
                        .num_columns(2)
                        .show(ui_layout, |ui_grid| {


                            egui::ComboBox::from_id_source("settings_theme_combo_box")
                                .width(200.0)
                                .selected_text(state.active_theme.name())
                                .show_ui(ui_grid, |ui_combobox| {
                                    for theme in themes {
                                        let res: egui::Response = ui_combobox.selectable_value(
                                            &mut state.active_theme,
                                            Rc::clone(theme),
                                            theme.name(),
                                        );
                                        if res.changed() {
                                            println!("Theme changed to '{}'", theme.name());
                                            ui_combobox
                                                .ctx()
                                                .set_style(state.active_theme.custom_style());
                                        }
                                    }
                                });


                        });
                },
            );
        });
}
