use crate::app::App;
use ddk::bitcoin;
use ddk::types::{GetValue, OutPoint};
use eframe::egui;

pub struct MemPoolExplorer {
    current: usize,
}

impl Default for MemPoolExplorer {
    fn default() -> Self {
        Self { current: 0 }
    }
}

impl MemPoolExplorer {
    pub fn show(&mut self, app: &mut App, ui: &mut egui::Ui) {
        let transactions = app.node.get_all_transactions().unwrap_or(vec![]);
        let utxos = app.wallet.get_utxos().unwrap_or_default();
        egui::SidePanel::left("transaction_picker")
            .resizable(false)
            .show_inside(ui, |ui| {
                ui.heading("Transactions");
                ui.separator();
                egui::Grid::new("transactions")
                    .striped(true)
                    .show(ui, |ui| {
                        ui.monospace("txid");
                        ui.monospace("value out");
                        ui.monospace("fee");
                        ui.end_row();
                        for (index, transaction) in transactions.iter().enumerate() {
                            let value_out: u64 = transaction
                                .transaction
                                .outputs
                                .iter()
                                .map(GetValue::get_value)
                                .sum();
                            let value_in: u64 = transaction
                                .transaction
                                .inputs
                                .iter()
                                .map(|input| utxos.get(input).map(GetValue::get_value))
                                .sum::<Option<u64>>()
                                .unwrap_or(0);
                            let txid = &format!("{}", transaction.transaction.txid())[0..8];
                            if value_in >= value_out {
                                let fee = value_in - value_out;
                                ui.selectable_value(&mut self.current, index, format!("{txid}"));
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Max),
                                    |ui| {
                                        let value_out = bitcoin::Amount::from_sat(value_out);
                                        ui.monospace(format!("{value_out}"));
                                    },
                                );
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Max),
                                    |ui| {
                                        let fee = bitcoin::Amount::from_sat(fee);
                                        ui.monospace(format!("{fee}"));
                                    },
                                );
                                ui.end_row();
                            } else {
                                ui.selectable_value(&mut self.current, index, format!("{txid}"));
                                ui.monospace("invalid");
                                ui.end_row();
                            }
                        }
                    });
            });
        if let Some(transaction) = transactions.get(self.current) {
            egui::SidePanel::left("inputs")
                .resizable(false)
                .show_inside(ui, |ui| {
                    ui.heading("Inputs");
                    ui.separator();
                    egui::Grid::new("inputs").striped(true).show(ui, |ui| {
                        ui.monospace("kind");
                        ui.monospace("outpoint");
                        ui.monospace("value");
                        ui.end_row();
                        for input in &transaction.transaction.inputs {
                            let (kind, hash, vout) = match input {
                                OutPoint::Regular { txid, vout } => {
                                    ("regular", format!("{txid}"), *vout)
                                }
                                OutPoint::Deposit(outpoint) => {
                                    ("deposit", format!("{}", outpoint.txid), outpoint.vout)
                                }
                                OutPoint::Coinbase { merkle_root, vout } => {
                                    ("coinbase", format!("{merkle_root}"), *vout)
                                }
                            };
                            let output = &utxos[input];
                            let hash = &hash[0..8];
                            let value = bitcoin::Amount::from_sat(output.get_value());
                            ui.monospace(format!("{kind}",));
                            ui.monospace(format!("{hash}:{vout}",));
                            ui.monospace(format!("{value}",));
                            ui.end_row();
                        }
                    });
                });
            egui::SidePanel::left("outputs")
                .resizable(false)
                .show_inside(ui, |ui| {
                    ui.heading("Outputs");
                    ui.separator();
                    egui::Grid::new("inputs").striped(true).show(ui, |ui| {
                        ui.monospace("vout");
                        ui.monospace("address");
                        ui.monospace("value");
                        ui.end_row();
                        for (vout, output) in transaction.transaction.outputs.iter().enumerate() {
                            let address = &format!("{}", output.address)[0..8];
                            let value = bitcoin::Amount::from_sat(output.get_value());
                            ui.monospace(format!("{vout}"));
                            ui.monospace(format!("{address}"));
                            ui.monospace(format!("{value}"));
                            ui.end_row();
                        }
                    });
                });
            egui::CentralPanel::default().show_inside(ui, |ui| {
                ui.heading("Viewing");
                ui.separator();
                let txid = transaction.transaction.txid();
                ui.monospace(format!("{txid}"));
            });
        } else {
            egui::CentralPanel::default().show_inside(ui, |ui| {
                ui.heading("No transactions in mempool");
            });
        }
    }
}
