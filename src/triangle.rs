use crate::framebuffer::Framebuffer;

pub type Vec2 = [f32; 2];

fn edge(a: Vec2, b: Vec2, p: Vec2) -> f32 {
    (b[0] - a[0]) * (p[1] - a[1]) - (b[1] - a[1]) * (p[0] - a[0])
}

/// Rellena un triángulo usando coordenadas baricéntricas.
pub fn triangle(fb: &mut Framebuffer, a: Vec2, b: Vec2, c: Vec2) {
    let area = edge(a, b, c);
    if area.abs() < f32::EPSILON {
        return;
    }

    let min_x = a[0].min(b[0]).min(c[0]).floor().max(0.0) as i32;
    let max_x = a[0].max(b[0]).max(c[0]).ceil().min((fb.width - 1) as f32) as i32;
    let min_y = a[1].min(b[1]).min(c[1]).floor().max(0.0) as i32;
    let max_y = a[1].max(b[1]).max(c[1]).ceil().min((fb.height - 1) as f32) as i32;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let p = [x as f32 + 0.5, y as f32 + 0.5];
            let w0 = edge(b, c, p) / area;
            let w1 = edge(c, a, p) / area;
            let w2 = edge(a, b, p) / area;
            if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                fb.point(x, y);
            }
        }
    }
}
