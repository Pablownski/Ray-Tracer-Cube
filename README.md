# Cubo con Raytracing

Raytracer en CPU escrito en Rust. Renderiza un cubo misterioso con textura
de runas y brillo pulsante, sobre una escena de cubos con cámara orbital.

## Ejecutar

```bash
cargo run --release
```

Al iniciar, genera la textura procedural en `assets/cube_texture.png`.

## Controles

| Control | Acción |
|---|---|
| `←` / `→` | Orbitar horizontalmente |
| `↑` / `↓` | Orbitar verticalmente |
| `Q` / `E` | Acercar / alejar |
| `R` | Restablecer cámara |
| `Escape` | Salir |

## Características

- Intersección rayo-cubo mediante *slab method* (AABB), con coordenadas UV
  por cara para el mapeo de texturas.
- Selección del impacto más cercano entre todos los cubos de la escena.
- Iluminación difusa Lambert combinada con un término emisivo pulsante: el
  cubo principal irradia luz intermitente del color de su propia textura,
  además de recibir la luz difusa de la escena.
- Textura procedural de mosaico de runas generada en código (sin depender de
  un archivo de imagen externo) y exportada a `assets/cube_texture.png`.
- Cámara orbital con yaw, pitch y radio.
- Fondo procedural de atardecer.
- Framebuffer calculado completamente en CPU (`Vec<u32>`).
- Modo opcional `--gif <directorio> <frames>` que renderiza un barrido
  orbital a frames PPM, listos para ensamblar externamente (p. ej. con
  ffmpeg) en un GIF de demostración.

## Escena

Un cubo con textura de runas y brillo pulsante, apoyado sobre el césped y
rodeado de rocas y árboles cúbicos, con un fondo degradado de atardecer.

<!-- Agregar aquí la captura final para Discord -->
