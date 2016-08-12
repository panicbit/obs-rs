#[macro_use]
extern crate obs;
use obs::InputSource;
use obs::gs::{self, ImageFile, Effect};

obs_module!(init);

fn init() -> bool {
    DemoSource::register();

    true
}

struct DemoSource {
    image: ImageFile
}

impl InputSource for DemoSource {
    fn id() -> &'static str { "DemoSource" }
    fn new() -> Self {
        DemoSource {
            image: ImageFile::open("ferris.png")
        }
    }

    fn width(&self) -> u32 {
        self.image.width()
    }

    fn height(&self) -> u32 {
        self.image.height()
    }

    fn render(&self, effect: Option<&mut Effect>) {
        if let Some(mut effect) = effect {
            if let Some(mut param) = effect.param_by_name("image") {
                param.set_texture(&self.image.texture());
                gs::draw_sprite(&self.image.texture(), self.image.width(), self.image.height());
            }
        }
    }
}
