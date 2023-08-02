use crate::app::App;
use crate::thunder::Thunder;
use ddk::bitcoin;
use ddk::types::{GetValue, OutPoint, Output};
use eframe::egui;
use std::collections::HashSet;

#[derive(Default)]
pub struct UtxoSelector {
    pub selected: HashSet<OutPoint>,
}

impl UtxoSelector {
    pub fn show(&mut self, app: &mut App, ui: &mut egui::Ui) {
        ui.heading("Spend UTXO");
        let utxos = &app.utxos;
        let total: u64 = utxos
            .iter()
            .filter(|(outpoint, _)| !self.selected.contains(outpoint))
            .map(|(_, output)| output.get_value())
            .sum();
        let mut utxos: Vec<_> = utxos.into_iter().collect();
        utxos.sort_by_key(|(outpoint, _)| format!("{outpoint}"));
        ui.separator();
        ui.monospace(format!("Total: {}", bitcoin::Amount::from_sat(total)));
        ui.separator();
        egui::Grid::new("utxos").striped(true).show(ui, |ui| {
            for (outpoint, output) in utxos {
                if self.selected.contains(outpoint) {
                    continue;
                }
                ui.horizontal(|ui| {
                    show_utxo(ui, outpoint, output);
                    if ui
                        .add_enabled(
                            !self.selected.contains(outpoint),
                            egui::Button::new("spend"),
                        )
                        .clicked()
                    {
                        self.selected.insert(*outpoint);
                    }
                });
                ui.end_row();
            }
        });
    }
}

pub fn show_utxo(ui: &mut egui::Ui, outpoint: &OutPoint, output: &Output<Thunder>) {
    let (kind, hash, vout) = match outpoint {
        OutPoint::Regular { txid, vout } => ("regular", format!("{txid}"), *vout),
        OutPoint::Deposit(outpoint) => ("deposit", format!("{}", outpoint.txid), outpoint.vout),
        OutPoint::Coinbase { merkle_root, vout } => ("coinbase", format!("{merkle_root}"), *vout),
    };
    let hash = &hash[0..8];
    ui.monospace(format!(
        "{kind} {hash} {vout}: {}",
        bitcoin::Amount::from_sat(output.get_value())
    ));
}
