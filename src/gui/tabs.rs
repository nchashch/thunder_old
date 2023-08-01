use eframe::egui;
use strum::IntoEnumIterator;

pub struct Tabs<T: IntoEnumIterator> {
    pub current: T,
}

impl<T: IntoEnumIterator + Eq + std::fmt::Display> Tabs<T> {
    pub fn new(current: T) -> Self {
        Self { current }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        for tab in T::iter() {
            if tab == self.current {
                ui.add(egui::Button::new(format!("{tab}")).fill(egui::Color32::BLACK));
            } else {
                if ui.add(egui::Button::new(format!("{tab}"))).clicked() {
                    self.current = tab;
                }
            }
        }
    }
}
