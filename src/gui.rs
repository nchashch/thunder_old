use crate::app::App;
use eframe::egui;

pub struct EguiApp {
    app: App,
    set_seed: SetSeed,
}

impl EguiApp {
    pub fn new(app: App, _cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        Self {
            app,
            set_seed: SetSeed::default(),
        }
    }
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Window::new("Set Seed").show(ctx, |ui| {
                self.set_seed.show(&mut self.app, ui);
            });
        });
    }
}

struct SetSeed {
    seed: String,
    passphrase: String,
}

impl Default for SetSeed {
    fn default() -> Self {
        Self {
            seed: "".into(),
            passphrase: "".into(),
        }
    }
}

impl SetSeed {
    fn show(&mut self, app: &App, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let seed_edit = egui::TextEdit::singleline(&mut self.seed)
                .hint_text("seed")
                .clip_text(false);
            ui.add(seed_edit);
            if ui.button("generate").clicked() {
                let mnemonic =
                    bip39::Mnemonic::new(bip39::MnemonicType::Words12, bip39::Language::English);
                self.seed = mnemonic.phrase().into();
            }
        });
        let passphrase_edit = egui::TextEdit::singleline(&mut self.passphrase)
            .hint_text("passphrase")
            .password(true)
            .desired_width(f32::INFINITY)
            .clip_text(false);
        ui.add(passphrase_edit);
        if ui.button("set").clicked() {
            let mnemonic =
                bip39::Mnemonic::from_phrase(&self.seed, bip39::Language::English).unwrap();
            let seed = bip39::Seed::new(&mnemonic, &self.passphrase);
            app.wallet
                .set_seed(seed.as_bytes().try_into().unwrap())
                .unwrap();
        }
    }
}
