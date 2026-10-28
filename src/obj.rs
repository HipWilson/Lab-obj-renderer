use std::fs;

pub type Vec3 = [f32; 3];

pub struct Model {
    pub vertices: Vec<Vec3>,
    pub indices: Vec<usize>,
}

impl Model {
    pub fn load(path: &str) -> Result<Model, String> {
        let text = fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for line in text.lines() {
            let mut parts = line.split_whitespace();
            match parts.next() {
                Some("v") => {
                    let c: Vec<f32> = parts.take(3).filter_map(|s| s.parse().ok()).collect();
                    if c.len() == 3 {
                        vertices.push([c[0], c[1], c[2]]);
                    }
                }
                Some("f") => {
                    let face: Vec<usize> = parts
                        .filter_map(|tok| tok.split('/').next()?.parse::<i64>().ok())
                        .map(|i| if i < 0 { (vertices.len() as i64 + i) as usize } else { (i - 1) as usize })
                        .collect();
                    // Triangulación en abanico para caras con más de 3 vértices
                    for k in 1..face.len().saturating_sub(1) {
                        indices.extend_from_slice(&[face[0], face[k], face[k + 1]]);
                    }
                }
                _ => {}
            }
        }

        if vertices.is_empty() || indices.is_empty() {
            return Err(format!("{path}: el modelo no contiene geometría"));
        }
        Ok(Model { vertices, indices })
    }
}
