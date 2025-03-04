# Calculadora en Rust

Este proyecto es una calculadora de línea de comandos escrita en Rust. Permite realizar operaciones aritméticas básicas y funciones adicionales como potencia, raíz cuadrada y módulo. La calculadora está diseñada de manera modular, con cada operación implementada en funciones separadas y utilizando la estructura `match` para seleccionar la operación correspondiente.

## Características

- **Operaciones Básicas:** Suma, resta, multiplicación y división.
- **Funciones Adicionales:** Potencia, raíz cuadrada y módulo.
- **Validación de Errores:** Manejo de errores como división por cero y cálculo de raíz cuadrada de números negativos.
- **Interfaz Interactiva:** Entrada de datos mediante la terminal para una experiencia sencilla y directa.

## Requisitos

- [Rust](https://www.rust-lang.org/) (versión estable)
- Cargo (incluido con la instalación de Rust)

## Instalación y Compilación

### Compilación para Linux

Para compilar el proyecto en modo Release en Linux, ejecuta:

```bash
cargo build --release
