# AGENTS.md — Simulador de Mundiales de Fútbol

Este archivo establece las reglas obligatorias para cualquier cambio en el proyecto.

## 1. Arquitectura y modularidad

- Respetar la arquitectura **Model–View–Controller (MVC)** existente:
  - `src/modelo/`: estado, entidades y reglas de la simulación.
  - `src/renderizacion/`: representación gráfica con `egui`.
  - `src/controladores/`: coordinación entre vistas, eventos y modelo.
  - `src/common/`: contratos compartidos, eventos y definiciones de ventanas.
  - `src/templates/`: componentes de interfaz reutilizables.
- Mantener los **eventos como `enum` tipados** y las **ventanas como `struct`**. Reutilizar los traits `Ventana`, `VentanaInfo` y `Controlador` en vez de crear mecanismos paralelos.
- Separar responsabilidades por módulos; evitar dependencias circulares conceptuales, duplicación de lógica y abstracciones innecesarias.
- Procurar que cada función tenga **como máximo 50 líneas**. Se toleran **hasta 60 líneas** únicamente cuando dividirla perjudique la claridad. Las funciones de **más de 60 líneas deben dividirse** en funciones más pequeñas con responsabilidades concretas.
- Mantener las convenciones de nombres y la organización existentes. No introducir patrones nuevos sin una necesidad concreta y justificada.

## 2. Rust idiomático y eficiencia

- Priorizar **corrección, seguridad, eficiencia y legibilidad**, aprovechando las características del lenguaje y de la biblioteca estándar.
- Reutilizar los traits estándar cuando expresen correctamente la operación: `Read`, `Write`, `Error`, `From`/`Into`, `TryFrom`/`TryInto`, `Deref`/`DerefMut`, `Add`, `Mul`, `Iterator`, `Default`, `Display`, entre otros.
- No crear funciones duplicadas que reimplementen operaciones ya expresadas por traits o abstracciones existentes.
- **No forzar la implementación de traits** por conveniencia: usarlos solamente cuando sus contratos y semántica correspondan. En particular, reservar `Deref`/`DerefMut` para tipos con una relación de acceso coherente al valor subyacente.
- Preferir préstamos (`&T`, `&mut T`) y estructuras apropiadas antes que clonaciones, asignaciones o conversiones innecesarias.
- Usar `Result`, `Option`, `?` y errores tipados para los casos recuperables; evitar `unwrap()`, `expect()` y `panic!()` en rutas normales de ejecución.
- Evitar `unsafe` salvo necesidad demostrable, documentada y revisada.
- No incorporar dependencias nuevas ni modificar interfaces públicas sin justificarlo previamente.
- Preservar las reglas del simulador y su comportamiento salvo que el cambio solicitado implique modificarlas.

## 3. Formato de entrega de cambios

- Indicar siempre las **rutas relativas a la raíz del repositorio**.
- **Cambios pequeños o localizados:** entregar bloques exactos para agregar, reemplazar o eliminar, indicando la ruta relativa del archivo.
- **Cambios grandes:** entregar un archivo `.patch` compatible con `git apply`, nunca un ZIP completo. Se considera un cambio grande cuando:
  - Se modifican **más de 10 archivos**, independientemente de la cantidad de líneas.
  - Se modifican **cientos de líneas en un único archivo**.
  - El volumen total de modificaciones hace poco práctico aplicar los cambios manualmente, aunque se afecten 10 archivos o menos.
- **Criterio flexible:** evaluar conjuntamente la cantidad de archivos afectados, las líneas modificadas y la complejidad de aplicar los cambios. No establecer una relación matemática rígida.
- Para aplicar un parche, indicar los comandos:

  ```bash
  git apply --check cambio.patch
  git apply cambio.patch
  ```

- Limitar los cambios al alcance solicitado; no reformatear ni refactorizar código ajeno a la tarea.
- No modificar archivos del repositorio ni realizar commits o push sin autorización expresa.

## 4. Documentación con Rustdoc

- Documentar en **español**, de forma **clara, precisa y concisa**, las interfaces públicas y la lógica no evidente.
- Usar `///` para elementos y `//!` para documentación de módulos cuando aporte contexto.
- Aprovechar las capacidades de Rustdoc: enlaces a elementos (por ejemplo, [`Result`]), fragmentos de código y ejemplos ejecutables (*doctests*).
- Añadir las secciones `# Examples`, `# Errors`, `# Panics` y `# Safety` cuando correspondan; no incluir secciones vacías ni comentarios que simplemente repitan el nombre de la función.
- Mantener la documentación sincronizada con el comportamiento real del código.

## 5. Validación y pruebas

- Acompañar correcciones y nuevas reglas de negocio con pruebas unitarias o de integración adecuadas, especialmente para grupos, clasificación, resultados y eliminatorias.
- Usar entradas deterministas o semillas controladas en las pruebas que involucren aleatoriedad.
- Comprobar cuando el entorno lo permita:

  ```bash
  cargo fmt --all -- --check
  cargo clippy --all-targets -- -D warnings
  cargo test
  cargo doc --no-deps
  ```

- **Nunca afirmar que una prueba o comando se ejecutó si no se ejecutó**. Informar los errores y las verificaciones pendientes.

## 6. Criterios generales

- Responder y escribir documentación y mensajes de cambios en **español**, respetando los nombres de API propios de Rust.
- Priorizar soluciones simples, reutilizables y consistentes con el proyecto existente.
- Antes de proponer un cambio, revisar las estructuras, enums, traits y utilidades disponibles para evitar implementaciones redundantes.
