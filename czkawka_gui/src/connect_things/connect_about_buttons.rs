use gtk4::prelude::*;
use log::error;

use crate::gui_structs::gui_data::GuiData;

const SPONSOR_SITE: &str = "https://github.com/sponsors/qarmin";
const REPOSITORY_SITE: &str = "https://github.com/qarmin/czkawka";
const INSTRUCTION_SITE: &str = "https://github.com/qarmin/czkawka/blob/master/instructions/Instruction.md";
const TRANSLATION_SITE: &str = "https://crwd.in/czkawka";

pub(crate) fn connect_about_buttons(gui_data: &GuiData) {
    let button_donation = gui_data.about.button_donation.clone();
    button_donation.connect_clicked(move |_| {
        if let Err(e) = open::that(SPONSOR_SITE) {
            error!("Failed to open sponsor site: {SPONSOR_SITE}, reason {e}");
        };
    });

    let button_instruction = gui_data.about.button_instruction.clone();
    button_instruction.connect_clicked(move |_| {
        if let Err(e) = open::that(INSTRUCTION_SITE) {
            error!("Failed to open instruction site: {INSTRUCTION_SITE}, reason {e}");
        };
    });

    let button_repository = gui_data.about.button_repository.clone();
    button_repository.connect_clicked(move |_| {
        if let Err(e) = open::that(REPOSITORY_SITE) {
            error!("Failed to open repository site: {REPOSITORY_SITE}, reason {e}");
        };
    });

    let button_translation = gui_data.about.button_translation.clone();
    button_translation.connect_clicked(move |_| {
        if let Err(e) = open::that(TRANSLATION_SITE) {
            error!("Failed to open translation site: {TRANSLATION_SITE}, reason {e}");
        };
    });
}
