# YieldDistributor - Soroban Smart Contract

Este repositorio contiene un contrato inteligente llamado **YieldDistributor** desarrollado en Rust para la red Soroban de Stellar. El contrato está diseñado para distribuir rendimientos (yield) a los usuarios de manera eficiente y transparente.

## Características principales

- **Lenguaje:** Rust
- **Plataforma:** Soroban (Stellar Smart Contracts)
- **Contrato:** YieldDistributor

## Funcionalidad del contrato

El contrato **YieldDistributor** implementa las siguientes funciones principales:

- `accrue`: Calcula y distribuye el rendimiento acumulado a los usuarios según sus saldos y la estrategia configurada.
- `update_strategy`: Permite actualizar la estrategia de cálculo de rendimiento (por ejemplo, cambiar el APR, modificar parámetros de distribución, etc).

## Estructura del proyecto

```text
.
├── contracts
│   └── yield_distributor
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

- Los contratos Soroban se ubican en la carpeta `contracts`, cada uno en su propio directorio.
- El contrato principal de este repositorio es `yield_distributor`.

## Requisitos

- [Rust](https://www.rust-lang.org/tools/install)
- [soroban-cli](https://github.com/stellar/soroban-tools)
- [cargo-make](https://sagiegurari.github.io/cargo-make/) (opcional para automatizar tareas)

## Instalación y uso

1. **Clona el repositorio:**
   ```sh
   git clone https://github.com/tu-usuario/yield-distributor.git
   cd yield-distributor
   ```

2. **Instala las dependencias:**
   ```sh
   rustup target add wasm32-unknown-unknown
   cargo install --locked soroban-cli
   ```

3. **Compila el contrato:**
   ```sh
   cd contracts/yield_distributor
   cargo build --target wasm32-unknown-unknown --release
   ```

4. **Despliega el contrato en Soroban:**
   Consulta la [documentación oficial de Soroban](https://soroban.stellar.org/docs) para desplegar el contrato usando `soroban-cli`.

## Ejemplo de uso

```rust
// Llama a accrue para distribuir el rendimiento
yield_distributor.accrue(env);

// Actualiza la estrategia de rendimiento
yield_distributor.update_strategy(env, nueva_estrategia);
```

## Pruebas

Para ejecutar las pruebas unitarias:
```sh
cargo test
```

## Contribución

¡Las contribuciones son bienvenidas! Por favor abre un issue o pull request para sugerencias y mejoras.

## Licencia

Este proyecto está bajo la licencia MIT.

---

Desarrollado con ❤️ usando Rust