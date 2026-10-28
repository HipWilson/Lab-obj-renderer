mod framebuffer;
mod obj;
mod triangle;

use framebuffer::Framebuffer;
use obj::Model;
use raylib::prelude::*;
use triangle::{triangle, Vec2};

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;
const BACKGROUND: Color = Color::new(18, 18, 28, 255);
const LINE_COLOR: Color = Color::new(90, 190, 255, 255);
const MARGIN: f32 = 0.8;

/// Escala y traslada (x, y) para que el modelo quede centrado y completamente visible.
fn fit_to_screen(model: &Model) -> Vec<Vec2> {
    let (mut min, mut max) = ([f32::MAX; 2], [f32::MIN; 2]);
    for v in &model.vertices {
        for i in 0..2 {
            min[i] = min[i].min(v[i]);
            max[i] = max[i].max(v[i]);
        }
    }
    let scale = MARGIN * (WIDTH as f32 / (max[0] - min[0])).min(HEIGHT as f32 / (max[1] - min[1]));
    let center = [(min[0] + max[0]) / 2.0, (min[1] + max[1]) / 2.0];

    model
        .vertices
        .iter()
        .map(|v| {
            [
                WIDTH as f32 / 2.0 + (v[0] - center[0]) * scale,
                HEIGHT as f32 / 2.0 - (v[1] - center[1]) * scale,
            ]
        })
        .collect()
}

fn render(fb: &mut Framebuffer, model: &Model, screen: &[Vec2]) {
    fb.clear(BACKGROUND);
    fb.set_color(LINE_COLOR);

    for t in model.indices.chunks_exact(3) {
        triangle(fb, screen[t[0]], screen[t[1]], screen[t[2]]);
    }
}

fn main() {
    let path = std::env::args()
        .skip(1)
        .find(|a| !a.starts_with("--"))
        .unwrap_or_else(|| "assets/torus.obj".to_string());
    let model = Model::load(&path).unwrap_or_else(|e| {
        eprintln!("Error al cargar el modelo: {e}");
        std::process::exit(1);
    });
    let screen = fit_to_screen(&model);

    let mut fb = Framebuffer::new(WIDTH, HEIGHT, BACKGROUND);
    render(&mut fb, &model, &screen);

    if std::env::args().any(|a| a == "--export") {
        fb.image.export_image("screenshot.png");
        return;
    }

    let (mut rl, thread) = raylib::init().size(WIDTH, HEIGHT).title("OBJ Renderer").build();
    let texture = rl.load_texture_from_image(&thread, &fb.image).expect("no se pudo crear la textura");

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(BACKGROUND);
        d.draw_texture(&texture, 0, 0, Color::WHITE);
    }
}