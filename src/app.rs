use egui::{Align};
use egui_modal::{Modal};

/// We derive Deserialize/Serialize so that we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct DifficultyEditorApp {
    #[serde(skip)]
    new_difficulty: String,
}

impl Default for DifficultyEditorApp {
    fn default() -> Self {
        Self {
            new_difficulty: "New Difficulty".to_owned()
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
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
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
                ui.text_edit_singleline(&mut self.new_difficulty);
            });
            new_difficulty_modal.buttons(ui, |ui| {
                // After clicking, the modal is automatically closed
                if new_difficulty_modal.suggested_button(ui, "Create").clicked() {
                    println!("New difficulty: {0}", self.new_difficulty);
                };
                new_difficulty_modal.button(ui, "Cancel");
            });
        });

        let about_modal = Modal::new(ctx, "about_modal")
            .with_close_on_outside_click(true);
        about_modal.show(|ui| {
            about_modal.title(ui, "About DRG Difficulty Editor");
            about_modal.frame(ui, |ui| {
                software_credits(ui);
            });
            about_modal.buttons(ui, |ui| {
                about_modal.button(ui, "Close");
            });
        });

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                ui.menu_button("File", |ui| {
                    if ui.button("New Difficulty").clicked() {
                        ui.close_menu();
                        new_difficulty_modal.open();
                    }

                    if !is_web {
                        ui.separator();
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    }
                });

                ui.menu_button("Help", |ui| {
                   if ui.button("About").clicked() {
                       ui.close_menu();
                       about_modal.open();
                   }
                });

                ui.add_space(16.0);

                egui::widgets::global_dark_light_mode_buttons(ui);
                ui.with_layout(egui::Layout::right_to_left(Align::Max), |ui| {
                    ui.add_space(16.0);
                    egui::warn_if_debug_build(ui);
                })
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("DRG Difficulty Editor");
        });
    }

    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
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
