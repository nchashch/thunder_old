use crate::app::App;
use ddk::bitcoin;
use eframe::egui;

pub struct UtxoCreator {
    utxo_type: UtxoType,
    value: String,
    address: String,
    main_address: String,
    main_fee: String,
}

#[derive(Eq, PartialEq)]
enum UtxoType {
    Regular,
    Withdrawal,
}

impl std::fmt::Display for UtxoType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Regular => write!(f, "regular"),
            Self::Withdrawal => write!(f, "withdrawal"),
        }
    }
}

impl Default for UtxoCreator {
    fn default() -> Self {
        Self {
            value: "".into(),
            address: "".into(),
            main_address: "".into(),
            main_fee: "".into(),
            utxo_type: UtxoType::Regular,
        }
    }
}

impl UtxoCreator {
    pub fn show(&mut self, app: &mut App, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Create");
            egui::ComboBox::from_id_source("utxo_type")
                .selected_text(format!("{}", self.utxo_type))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.utxo_type, UtxoType::Regular, "regular");
                    ui.selectable_value(&mut self.utxo_type, UtxoType::Withdrawal, "withdrawal");
                });
            ui.heading("UTXO");
        });
        ui.separator();
        ui.horizontal(|ui| {
            ui.monospace("Value:       ");
            ui.add(egui::TextEdit::singleline(&mut self.value));
            ui.monospace("BTC");
        });
        ui.horizontal(|ui| {
            ui.monospace("Address:     ");
            ui.add(egui::TextEdit::singleline(&mut self.address));
            ui.button("generate");
        });
        if self.utxo_type == UtxoType::Withdrawal {
            ui.horizontal(|ui| {
                ui.monospace("Main Address:");
                ui.add(egui::TextEdit::singleline(&mut self.main_address));
                ui.button("generate");
            });
            ui.horizontal(|ui| {
                ui.monospace("Main Fee:    ");
                ui.add(egui::TextEdit::singleline(&mut self.main_address));
                ui.monospace("BTC");
            });
        }
        ui.button("create");
    }
}
