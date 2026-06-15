# Yield Distributor Contract

El contrato inteligente **Yield Distributor**, desarrollado en Soroban, está diseñado para gestionar y distribuir rendimientos (Yield) de forma automatizada. Permite a los usuarios participar en estrategias de depósito personalizadas, donde cada estrategia está vinculada a un porcentaje de rendimiento definido por el administrador. Los usuarios pueden tener múltiples estrategias activas simultáneamente (identificadas mediante un `memo` único), facilitando el seguimiento de los diferentes depósitos, la acumulación de intereses (APY), y el cambio de estados desde la creación (Active) hasta su maduración (Expired) y posterior retiro (Completed).

---

## Funciones del Contrato

### Administración y Configuración

#### `initialize`

Inicializa el contrato asignando la cuenta de administrador.

- **Parámetros:**
  - `env`: Entorno de ejecución (`&Env`)
  - `admin`: Dirección del administrador (`Address`)

#### `set_pct`

Define el porcentaje de rendimiento global.

- **Parámetros:**
  - `env`: Entorno de ejecución (`Env`)
  - `pct`: Porcentaje de rendimiento (`i128`)

#### `set_token`

Define la dirección del token asociado al contrato de rendimiento.

- **Parámetros:**
  - `env`: Entorno de ejecución (`&Env`)
  - `token`: Dirección del contrato del token (`Address`)

---

### Estrategias de Usuario

#### `set_strat`

Crea o añade una nueva estrategia a la lista de estrategias de un usuario.

- **Parámetros:**
  - `env`: Entorno de ejecución (`Env`)
  - `user`: Dirección del usuario (`Address`)
  - `memo`: Identificador único de la estrategia (Timestamp de UNIX) (`u64`)
  - `amount`: Cantidad a depositar/asignar en la estrategia (`i128`)

#### `get_strat`

Obtiene una estrategia en particular dentro de la lista de estrategias del usuario.

- **Parámetros:**
  - `env`: Entorno de ejecución (`&Env`)
  - `user`: Dirección del usuario (`Address`)
  - `memo`: Identificador único de la estrategia (`u64`)
- **Retorno:** Retorna el objeto `Strategy` que coincide con el `memo`.

#### `get_all_strats`

Obtiene la lista completa de todas las estrategias asociadas a un usuario.

- **Parámetros:**
  - `env`: Entorno de ejecución (`&Env`)
  - `user`: Dirección del usuario (`Address`)
- **Retorno:** Retorna un arreglo `Vec<Strategy>`.

---

### Gestión de Estados de la Estrategia

#### `set_s_exp`

Cambia el estado de una estrategia específica a `Expired` y ejecuta la emisión (mint) de rendimientos.

- **Parámetros:**
  - `env`: Entorno de ejecución (`Env`)
  - `user`: Dirección del usuario (`Address`)
  - `memo`: Identificador único de la estrategia (`u64`)

#### `set_s_cmp`

Cambia el estado de una estrategia específica a `Completed`. Solo puede llamarse si la estrategia estaba previamente en estado `Expired`.

- **Parámetros:**
  - `env`: Entorno de ejecución (`Env`)
  - `user`: Dirección del usuario (`Address`)
  - `memo`: Identificador único de la estrategia (`u64`)

---

### Rendimiento (APY) y Retiros

#### `get_apy`

Consulta el balance del usuario en el token de la estrategia elegida.

- **Parámetros:**
  - `env`: Entorno de ejecución (`&Env`)
  - `user`: Dirección del usuario (`Address`)
  - `memo`: Identificador único de la estrategia (`u64`)
- **Retorno:** El balance acumulado (`i128`).

#### `accrue`

Añade rendimiento (apy) mediante la emisión (mint) de tokens para el usuario correspondiente a una de sus estrategias.

- **Parámetros:**
  - `env`: Entorno de ejecución (`Env`)
  - `user`: Dirección del usuario (`Address`)
  - `memo`: Identificador único de la estrategia (`u64`)
  - `amount`: Cantidad de rendimiento a añadir (`i128`)
- **Retorno:** `Result<(), YieldError>`

#### `withdraw`

Permite al usuario retirar un monto específico quemando (burn) los tokens correspondientes. La estrategia debe estar en estado `Completed`.

- **Parámetros:**
  - `env`: Entorno de ejecución (`Env`)
  - `amount`: Monto a retirar (`i128`)
  - `user`: Dirección del usuario (`Address`)
  - `memo`: Identificador único de la estrategia (`u64`)
- **Retorno:** `Result<(), YieldError>`

---

Aquí tienes un resumen de los cambios realizados:

- **Estructura Strategy**: Se agregó el campo `memo: u64` que servirá como un identificador único (con formato de timestamp de UNIX) para cada estrategia individual.
- **Función set_strat**: Ahora recibe `memo: u64` y en lugar de sobrescribir directamente el estado del Address con una sola estrategia, carga un `Vec<Strategy>`, le añade la nueva estrategia y guarda la lista completa.
- **Función get_strat**: Ahora requiere que le pases el parámetro `memo: u64` para buscar dentro de la lista de estrategias del usuario y retornar la que coincida exactamente con ese identificador.
- **Nueva función get_all_strats**: Creada específicamente para que devuelva la lista completa de `Vec<Strategy>` asignada a un determinado Address.
- **Funciones secundarias** (`set_s_exp`, `set_s_cmp`, `accrue`, `withdraw`, etc): Se ajustaron automáticamente para requerir el memo de tal forma que todo el código siga compilando correctamente.
