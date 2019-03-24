extern crate azul;

use azul::prelude::*;

struct MyDataModel { }

impl Layout for MyDataModel {
    fn layout(&self, _: LayoutInfo<Self>) -> Dom<Self> {
        Dom::label("This window should be half-transparent")
    }
}

fn main() {
    let app_config = AppConfig { background_color: ColorU { r: 255, g: 0, b: 0, a: 40 }, .. Default::default() };
    let mut app = App::new(MyDataModel { }, app_config).unwrap();
    let window = app.create_window(WindowCreateOptions::default(), css::native()).unwrap();
    let window2 = app.create_window(WindowCreateOptions::default(), css::native()).unwrap();
    let window3 = app.create_window(WindowCreateOptions::default(), css::native()).unwrap();
    app.add_window(window2);
    app.add_window(window3);
    app.run(window).unwrap();
}