# Cubo con Raytracing

Raytracer en CPU escrito en Rust. Renderiza "El Santuario del Cubo", una escena
de cubos con iluminación difusa Lambert y cámara orbital.

## Ejecutar

```bash
cargo run --release
```

## Controles

| Control | Acción |
|---|---|
| `←` / `→` | Orbitar horizontalmente |
| `↑` / `↓` | Orbitar verticalmente |
| `Q` / `E` | Acercar / alejar |
| `R` | Restablecer cámara |
| `Escape` | Salir |

## Características

- Intersección rayo-cubo mediante *slab method* (AABB).
- Selección del impacto más cercano entre todos los cubos de la escena.
- Iluminación difusa Lambert exclusivamente (sin specular, reflejos ni
  refracción).
- Cámara orbital con yaw, pitch y radio, y render bajo demanda (solo se
  recalcula el frame cuando la cámara cambia).
- Fondo procedural de atardecer.
- Framebuffer calculado completamente en CPU (`Vec<u32>`), sin GPU, shaders ni
  motores gráficos.

## Escena: El Santuario del Cubo

Un cubo morado sobre un pedestal escalonado de piedra, con rocas y árboles
cúbicos alrededor y un fondo degradado de atardecer.

<!-- Agregar aquí la captura final para Discord -->
