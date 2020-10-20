use nannou::prelude::*;

struct Model {
    texture: wgpu::Texture,
}

fn main() {
    nannou::app(model).run();
}

fn model(app: &App) -> Model {
    // create a new window
    app.new_window().size(512, 512).view(view).build().unwrap();
    // load image from disk and upload
    let assets = app.assets_path().unwrap();
    let img_path = assets.join("images").join("nature").join("nature_2.jpg");
    let texture = wgpu::Texture::from_path(app, img_path).unwrap();
    Model { texture }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);
    let draw = app.draw();

    // BASIC DRAWING AN IMAGE
    // draw.texture(&model.texture);

    // SCALED DRAWING
    let win = app.window_rect();
    let r = Rect::from_w_h(100.0, 100.0).top_left_of(win);
    draw.texture(&model.texture).xy(r.xy()).wh(r.wh());

    // FOR ALL DRAWING
    draw.to_frame(app, &frame).unwrap();
}
