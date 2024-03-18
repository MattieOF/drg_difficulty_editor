use crate::difficulty::Difficulty;
use egui::{Align, OpenUrl, Vec2, Visuals};
use egui_modal::Modal;
use linked_hash_map::LinkedHashMap;

/// We derive Deserialize/Serialize so that we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct DifficultyEditorApp {
    #[serde(skip)]
    new_difficulty_name: String,
    #[serde(skip)]
    new_difficulty_base: String,
    #[serde(skip)]
    dark_mode_enabled: bool,
    #[serde(skip)]
    project_open: bool,

    #[serde(skip)]
    base_difficulties: LinkedHashMap<String, Difficulty>,
}

impl Default for DifficultyEditorApp {
    fn default() -> Self {
        Self {
            new_difficulty_name: "New Difficulty".to_owned(),
            new_difficulty_base: "Haz5".to_owned(),
            dark_mode_enabled: true,
            project_open: false,
            base_difficulties: Self::load_base_difficulties(),
        }
    }
}

impl DifficultyEditorApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            // cc.egui_ctx.send_viewport_cmd(ViewportCommand::Maximized(true));
            if eframe::get_value(storage, "dark_mode").unwrap_or(true) {
                cc.egui_ctx.set_visuals(Visuals::dark())
            } else {
                cc.egui_ctx.set_visuals(Visuals::light())
            }
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn reset_new_difficulty_modal(&mut self) {
        self.new_difficulty_name = "New Difficulty".to_owned();
    }

    fn load_base_difficulties() -> LinkedHashMap<String, Difficulty> {
        let mut map = LinkedHashMap::default();
        map.insert("Hazard 1".to_string(), Difficulty::from_name("Hazard 1"));
        map.insert("Hazard 2".to_string(), Difficulty::from_name("Hazard 2"));
        map.insert("Hazard 3".to_string(), Difficulty::from_name("Hazard 3"));
        map.insert("Hazard 4".to_string(), Difficulty::from_name("Hazard 4"));
        map.insert("Hazard 5".to_string(), Difficulty::from_name("Hazard 5"));
        map
    }
}

impl eframe::App for DifficultyEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        let is_web = cfg!(target_arch = "wasm32");

        // Initialise modals
        let new_difficulty_modal = Modal::new(ctx, "new_difficulty_modal");
        new_difficulty_modal.show(|ui| {
            new_difficulty_modal.title(ui, "Create New Difficulty");
            new_difficulty_modal.frame(ui, |ui| {
                egui::Grid::new("new_diff_grid")
                    .num_columns(2)
                    .spacing(Vec2::new(10.0, 6.0))
                    .show(ui, |ui| {
                        ui.label("Name:");
                        ui.text_edit_singleline(&mut self.new_difficulty_name);
                        ui.end_row();
                        ui.label("Base:");
                        egui::ComboBox::from_id_source("new_diff_base_selection")
                            .selected_text(
                                match self.base_difficulties.get(&self.new_difficulty_base) {
                                    None => "Select a difficulty".to_string(),
                                    Some(difficulty) => difficulty.name.to_owned(),
                                },
                            )
                            .show_ui(ui, |ui| {
                                for (key, difficulty) in &self.base_difficulties {
                                    ui.selectable_value(
                                        &mut self.new_difficulty_base,
                                        key.to_string(),
                                        difficulty.name.to_owned(),
                                    );
                                }
                            });
                    });
            });
            new_difficulty_modal.buttons(ui, |ui| {
                // After clicking, the modal is automatically closed
                if new_difficulty_modal
                    .suggested_button(ui, "Create")
                    .clicked()
                {
                    println!(
                        "New difficulty: {0}, with {1} as base",
                        self.new_difficulty_name, self.new_difficulty_base
                    );
                };
                new_difficulty_modal.button(ui, "Cancel");
            });
        });

        let about_modal = Modal::new(ctx, "about_modal").with_close_on_outside_click(true);
        about_modal.show(|ui| {
            about_modal.title(ui, "About DRG Difficulty Editor");
            ui.label("A tool to create custom DRG difficulties.");
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label("Created by ");
                ui.hyperlink_to("Matt Ware", "https://www.mattware.dev/");
                ui.label(". (");
                ui.hyperlink_to(
                    "mattbware3@gmail.com",
                    "mailto:mattbware3@gmail.com?body=Hello!",
                );
                ui.label(")");
            });
            ui.add_space(20.0);
            ui.label(egui::RichText::new("Software Credits").size(15.0));
            about_modal.frame(ui, |ui| {
                software_credits(ui);
            });
            about_modal.buttons(ui, |ui| {
                about_modal.button(ui, "Close");
            });
        });

        let options_modal = Modal::new(ctx, "options_modal").with_close_on_outside_click(true);
        options_modal.show(|ui| {
            options_modal.title(ui, "Options");
            options_modal.frame(ui, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink(true)
                    .max_height(400.0)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Theme: ");
                            let mut visuals = ui.ctx().style().visuals.clone();
                            visuals.light_dark_radio_buttons(ui);
                            self.dark_mode_enabled = visuals.dark_mode;
                            ui.ctx().set_visuals(visuals);
                        });
                    });
            });
            options_modal.buttons(ui, |ui| {
                options_modal.button(ui, "Close");
            });
        });

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                ui.menu_button("File", |ui| {
                    if ui.button("New Difficulty").clicked() {
                        ui.close_menu();
                        self.reset_new_difficulty_modal();
                        new_difficulty_modal.open();
                    }

                    if !is_web {
                        ui.separator();
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    }
                });

                ui.menu_button("Tools", |ui| {
                    if ui.button("Options").clicked() {
                        ui.close_menu();
                        options_modal.open();
                    }
                });

                ui.menu_button("Help", |ui| {
                    if ui.button("View Source").clicked() {
                        ui.close_menu();
                        ui.ctx().open_url(OpenUrl::new_tab(
                            "https://github.com/MattieOF/drg_difficulty_editor",
                        ));
                    }

                    if ui.button("About").clicked() {
                        ui.close_menu();
                        about_modal.open();
                    }
                });

                ui.add_space(16.0);

                ui.with_layout(egui::Layout::right_to_left(Align::Max), |ui| {
                    ui.add_space(16.0);
                    egui::warn_if_debug_build(ui);
                })
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if !self.project_open {
                ui.vertical_centered(|ui| {
                    ui.add_space(ui.available_height() * 0.45);
                    ui.label(
                        egui::RichText::new("DRG Difficulty Editor")
                            .size(40.0)
                            .strong(),
                    );
                    ui.add_space(20.0);
                    ui.horizontal(|ui| {
                        centerer(ui, |ui| {
                            if ui
                                .selectable_label(
                                    false,
                                    egui::RichText::new("New Difficulty").size(15.0),
                                )
                                .clicked()
                            {
                                self.reset_new_difficulty_modal();
                                new_difficulty_modal.open();
                            }

                            if ui
                                .selectable_label(
                                    false,
                                    egui::RichText::new("Open Difficulty").size(15.0),
                                )
                                .clicked()
                            {
                                self.reset_new_difficulty_modal();
                                new_difficulty_modal.open();
                            }
                        });
                    });
                });
                return;
            }
        });
    }

    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
        eframe::set_value(storage, "dark_mode", &self.dark_mode_enabled);
    }
}

fn software_credits(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Also uses ");
        ui.hyperlink_to("egui-modal", "https://github.com/n00kii/egui-modal");
        ui.label(".");
    });
}

// Massive thanks to Juan Campa (juancampa) for this function.
// Helper function to center arbitrary widgets. It works by measuring the width of the widgets after rendering, and
// then using that offset on the next frame.
fn centerer(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui)) {
    ui.horizontal(|ui| {
        let id = ui.id().with("_centerer");
        let last_width: Option<f32> = ui.memory_mut(|mem| mem.data.get_temp(id));
        if let Some(last_width) = last_width {
            ui.add_space((ui.available_width() - last_width) / 2.0);
        }
        let res = ui
            .scope(|ui| {
                add_contents(ui);
            })
            .response;
        let width = res.rect.width();
        ui.memory_mut(|mem| mem.data.insert_temp(id, width));

        // Repaint if width changed
        match last_width {
            None => ui.ctx().request_repaint(),
            Some(last_width) if last_width != width => ui.ctx().request_repaint(),
            Some(_) => {}
        }
    });
}
