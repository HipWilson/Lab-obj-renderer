# OBJ Renderer

Renderer de software en Rust que carga un modelo `.obj` y lo dibuja triángulo por triángulo con una función `triangle` propia. Raylib se usa únicamente para abrir la ventana y mostrar el framebuffer; no se usan sus funciones para cargar ni dibujar modelos.

## Pipeline

```
Archivo OBJ
    ↓
Vértices + índices
    ↓
Triángulos
    ↓
Líneas / relleno
    ↓
Pixeles
    ↓
Pantalla
```

## Estructura

- `src/obj.rs`: cargador de OBJ (vértices e índices de caras, triangulación de polígonos).
- `src/triangle.rs`: función `triangle` (relleno con coordenadas baricéntricas).
- `src/framebuffer.rs`: framebuffer con `point`.
- `src/main.rs`: escala/traslada el modelo al centro de la ventana y dibuja cada triángulo.
- `assets/torus.obj`: modelo utilizado.

## Ejecución

```
cargo run --release
```

Para usar otro modelo: `cargo run --release -- ruta/al/modelo.obj`

## Resultado

![Modelo renderizado](docs/render.png)
