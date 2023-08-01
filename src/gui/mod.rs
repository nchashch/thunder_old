use crate::app::App;
use eframe::egui;

mod deposit;
mod miner;
mod seed;
mod tabs;

use deposit::Deposit;
use miner::Miner;
use seed::SetSeed;
use tabs::Tabs;

pub struct EguiApp {
    app: App,
    set_seed: SetSeed,
    miner: Miner,
    deposit: Deposit,
    tabs: Tabs<Tab>,
}

#[derive(strum_macros::EnumIter, strum_macros::Display, Eq, PartialEq)]
enum Tab {
    #[strum(to_string = "transaction builder")]
    TransactionBuilder,
    #[strum(to_string = "mempool explorer")]
    MemPool,
}

impl EguiApp {
    pub fn new(app: App, cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let tabs = Tabs::new(Tab::TransactionBuilder);
        Self {
            app,
            set_seed: SetSeed::default(),
            miner: Miner::default(),
            deposit: Deposit::default(),
            tabs,
        }
    }
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.app.wallet.has_seed().unwrap_or(false) {
            egui::TopBottomPanel::top("tabs").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    self.tabs.show(ui);
                });
            });
            egui::TopBottomPanel::bottom("util").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    self.miner.show(&mut self.app, ui);
                    ui.separator();
                    self.deposit.show(&mut self.app, ui);
                });
            });
            egui::CentralPanel::default().show(ctx, |ui| match self.tabs.current {
                Tab::TransactionBuilder => {
                    egui::SidePanel::left("spend_utxo")
                        .exact_width(150.)
                        .resizable(false)
                        .show_inside(ui, |ui| {
                            ui.heading("Spend UTXO");
                        });
                    egui::SidePanel::right("create_utxo")
                        .exact_width(150.)
                        .resizable(false)
                        .show_inside(ui, |ui| {
                            ui.heading("Create UTXO");
                        });
                    egui::CentralPanel::default().show_inside(ui, |ui| {});
                }
                Tab::MemPool => {
                    egui::SidePanel::left("transaction_picker")
                        .exact_width(150.)
                        .resizable(false)
                        .show_inside(ui, |ui| {
                            ui.button("previous");
                            egui::Grid::new("transactions").show(ui, |ui| {
                                for i in 0..30 {
                                    ui.horizontal(|ui| {
                                        ui.monospace(format!("transaction {i}"));
                                    });
                                    ui.end_row();
                                }
                            });
                            ui.button("next");
                        });
                    egui::CentralPanel::default().show_inside(ui, |ui| {});
                }
            });
        } else {
            egui::CentralPanel::default().show(ctx, |_ui| {
                egui::Window::new("Set Seed").show(ctx, |ui| {
                    self.set_seed.show(&mut self.app, ui);
                });
            });
        }
    }
}
