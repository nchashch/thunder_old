use crate::app::App;
use ddk::bitcoin;
use ddk::types::GetValue;
use eframe::egui;

mod deposit;
mod miner;
mod seed;
mod utxo_creator;
mod utxo_selector;

use deposit::Deposit;
use miner::Miner;
use seed::SetSeed;
use utxo_selector::{show_utxo, UtxoSelector};

use self::utxo_creator::UtxoCreator;

pub struct EguiApp {
    app: App,
    set_seed: SetSeed,
    miner: Miner,
    deposit: Deposit,
    tab: Tab,
    utxo_selector: UtxoSelector,
    utxo_creator: UtxoCreator,
}

#[derive(Eq, PartialEq)]
enum Tab {
    TransactionBuilder,
    MemPool,
    BlockExplorer,
}

impl EguiApp {
    pub fn new(app: App, cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self {
            app,
            set_seed: SetSeed::default(),
            miner: Miner::default(),
            deposit: Deposit::default(),
            utxo_selector: UtxoSelector::default(),
            utxo_creator: UtxoCreator::default(),
            tab: Tab::TransactionBuilder,
        }
    }
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.app.wallet.has_seed().unwrap_or(false) {
            egui::TopBottomPanel::top("tabs").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(
                        &mut self.tab,
                        Tab::TransactionBuilder,
                        "transaction builder",
                    );
                    ui.selectable_value(&mut self.tab, Tab::MemPool, "mempool explorer");
                    ui.selectable_value(&mut self.tab, Tab::BlockExplorer, "block explorer");
                });
            });
            egui::TopBottomPanel::bottom("util").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    self.miner.show(&mut self.app, ui);
                    ui.separator();
                    self.deposit.show(&mut self.app, ui);
                });
            });
            egui::CentralPanel::default().show(ctx, |ui| match self.tab {
                Tab::TransactionBuilder => {
                    egui::SidePanel::left("spend_utxo")
                        .exact_width(250.)
                        .resizable(false)
                        .show_inside(ui, |ui| {
                            self.utxo_selector.show(&mut self.app, ui);
                        });
                    egui::SidePanel::left("value_in")
                        .exact_width(250.)
                        .resizable(false)
                        .show_inside(ui, |ui| {
                            ui.heading("Value In");
                            let utxos = &self.app.utxos;
                            let total: u64 = utxos
                                .iter()
                                .filter(|(outpoint, _)| {
                                    self.utxo_selector.selected.contains(outpoint)
                                })
                                .map(|(_, output)| output.get_value())
                                .sum();
                            let mut utxos: Vec<_> = utxos
                                .into_iter()
                                .filter(|(outpoint, _)| {
                                    self.utxo_selector.selected.contains(outpoint)
                                })
                                .collect();
                            utxos.sort_by_key(|(outpoint, _)| format!("{outpoint}"));
                            ui.separator();
                            ui.monospace(format!("Total: {}", bitcoin::Amount::from_sat(total)));
                            ui.separator();
                            egui::Grid::new("utxos").striped(true).show(ui, |ui| {
                                for (outpoint, output) in utxos {
                                    ui.horizontal(|ui| {
                                        show_utxo(ui, outpoint, output);
                                        if ui.button("remove").clicked() {
                                            self.utxo_selector.selected.remove(outpoint);
                                        }
                                    });
                                    ui.end_row();
                                }
                            });
                        });
                    egui::SidePanel::left("value_out")
                        .exact_width(250.)
                        .resizable(false)
                        .show_inside(ui, |ui| {
                            ui.heading("Value Out");
                            ui.separator();
                            ui.monospace("Total: 0 BTC");
                            ui.separator();
                        });
                    egui::SidePanel::left("create_utxo")
                        .exact_width(450.)
                        .resizable(false)
                        .show_separator_line(false)
                        .show_inside(ui, |ui| {
                            self.utxo_creator.show(&mut self.app, ui);
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
                Tab::BlockExplorer => {
                    egui::SidePanel::left("block_picker")
                        .exact_width(150.)
                        .resizable(false)
                        .show_inside(ui, |ui| {
                            ui.button("previous");
                            egui::Grid::new("blocks").show(ui, |ui| {
                                for i in 0..30 {
                                    ui.horizontal(|ui| {
                                        ui.monospace(format!("block {i}"));
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
