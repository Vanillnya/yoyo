use egui::{include_image, ImageSource};

pub struct Skin {
    pub and: ImageSource<'static>,
    pub nor: ImageSource<'static>,
}

impl Skin {
    pub fn new() -> Self {
        Self {
            and: include_image!(
                "../assets/skins/digital/wikipedia_inductiveload/AND_ANSI_Labelled.svg"
            ),
            nor: include_image!(
                "../assets/skins/digital/wikipedia_inductiveload/NOR_ANSI_Labelled.svg"
            ),
        }
    }
}
