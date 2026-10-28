mod framebuffer;
mod obj;
mod triangle;

use framebuffer::Framebuffer;
use obj::{Model, Vec3};
use raylib::prelude::*;
use triangle::{triangle, Vec2};

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;
const BACKGROUND: Color = Color::new(18, 18, 28, 255);
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

fn shade(tri: &[Vec3; 3]) -> Color {
    let u = [tri[1][0] - tri[0][0], tri[1][1] - tri[0][1], tri[1][2] - tri[0][2]];
    let v = [tri[2][0] - tri[0][0], tri[2][1] - tri[0][1], tri[2][2] - tri[0][2]];
    let n = [
        u[1] * v[2] - u[2] * v[1],
        u[2] * v[0] - u[0] * v[2],
        u[0] * v[1] - u[1] * v[0],
    ];
    let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt().max(f32::EPSILON);
    let light = (n[2] / len).abs() * 0.75 + 0.25;
    Color::new((90.0 * light) as u8, (190.0 * light) as u8, (255.0 * light) as u8, 255)
}

fn render(fb: &mut Framebuffer, model: &Model, screen: &[Vec2]) {
    fb.clear(BACKGROUND);

    // Los triángulos más lejanos se dibujan primero para que no se tapen mal entre sí
    let mut faces: Vec<usize> = (0..model.indices.len() / 3).collect();
    let depth = |f: usize| -> f32 { (0..3).map(|k| model.vertices[model.indices[f * 3 + k]][2]).sum() };
    faces.sort_by(|&a, &b| depth(a).partial_cmp(&depth(b)).unwrap());

    for f in faces {
        let idx = [model.indices[f * 3], model.indices[f * 3 + 1], model.indices[f * 3 + 2]];
        let tri3 = [model.vertices[idx[0]], model.vertices[idx[1]], model.vertices[idx[2]]];
        fb.set_color(shade(&tri3));
        triangle(fb, screen[idx[0]], screen[idx[1]], screen[idx[2]]);
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
