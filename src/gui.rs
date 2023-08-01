use crate::app::App;
use eframe::egui::{self};

pub struct EguiApp {
    app: App,
    set_seed: SetSeed,
    miner: Miner,
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
            miner: Miner::default(),
        }
    }
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |_ui| {
            if self.app.wallet.has_seed().unwrap_or(false) {
                egui::Window::new("Miner").show(ctx, |ui| {
                    self.miner.show(&mut self.app, ui);
                });
            } else {
                egui::Window::new("Set Seed").show(ctx, |ui| {
                    self.set_seed.show(&mut self.app, ui);
                });
            }
        });
    }
}

struct Miner;

impl Default for Miner {
    fn default() -> Self {
        Self
    }
}

impl Miner {
    fn show(&mut self, app: &mut App, ui: &mut egui::Ui) {
        let block_height = app.node.get_height().unwrap_or(0);
        let best_hash = app.node.get_best_hash().unwrap_or([0; 32].into());
        ui.add(egui::Label::new(format!("Block height: {block_height}")).wrap(false));
        ui.add(egui::Label::new(format!("Best hash: {best_hash}")).wrap(false));
        if ui.button("mine").clicked() {
            app.mine_tx
                .try_send(())
                .expect("failed to send () to mine_tx");
        }
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
            .clip_text(false);
        ui.add(passphrase_edit);
        let mnemonic = bip39::Mnemonic::from_phrase(&self.seed, bip39::Language::English);
        if ui
            .add_enabled(mnemonic.is_ok(), egui::Button::new("set"))
            .clicked()
        {
            let mnemonic = mnemonic.expect("should never happen");
            let seed = bip39::Seed::new(&mnemonic, &self.passphrase);
            app.wallet
                .set_seed(seed.as_bytes().try_into().expect("seed it not 64 bytes"))
                .expect("failed to set HD wallet seed");
        }
    }
}
