# Why - Compilación, configuración y ejecución

*Lea esto en otros idiomas: [English](../build.md), [日本語](build_JP.md), [Korean](build_KR.md), [简体中文](build_ZH-CN.md).*

## Plataformas soportadas

En un largo plazo, es probable que la mayoría de las plataformas sean compatibles en cierta medida.
El lenguaje de programación de Why `rust` ha compilado metas para la mayoría de las plataformas.

¿Qué funciona hasta ahora?

* Linux x86\_64 y MacOS [why + mining + development]
* Todavía no funciona con windows 10 [why kind-of builds. No mining yet. Help wanted!]

## Requisitos

* rust 1.34+ (usa [rustup]((https://www.rustup.rs/))- por ejemplo, `curl https://sh.rustup.rs -sSf | sh; source $HOME/.cargo/env`)
  * Si rust está instalado, puede simplemente actualizar la versión con  `rustup update`
* clang
* ncurses y libs (ncurses, ncursesw5)
* zlib libs (zlib1g-dev or zlib-devel)
* pkg-config
* libssl-dev
* linux-headers (reportado como necesario en Alpine linux)
* llvm

Para las distribuciones basadas en Debian (Debian, Ubuntu, Mint, etc), todo en un comando (exceptuando Rust):

```sh
apt install build-essential cmake git libgit2-dev clang libncurses5-dev libncursesw5-dev zlib1g-dev pkg-config libssl-dev llvm
```

Para las Mac:

```sh
xcode-select --install
brew install --with-toolchain llvm
brew install pkg-config
brew install openssl
```

## Pasos para la compilación

```sh
git clone https://github.com/mimblewimble/why.git
cd why
cargo build --release
```

Why también puede compilarse en modo debug (sin la etiqueta `--release`, pero usando la etiqueta `--debug` o `--verbose`) esto hará que la sincronización rápida sea excesivamente lenta debido a la gran sobrecarga de las operaciones criptográficas.

## Errores de compilación

Vea [Solución de problemas](https://github.com/mimblewimble/docs/wiki/Troubleshooting)

## ¿Qué se ha compilado?

Con una compilación finalizada se obtiene:

* `target/release/why` - los binarios principales de why

Todos los datos, configuración y archivos de registro creados y utilizados por Why se encuentran en el directorio oculto `~/.why` (bajo el directorio home del usuario) por defecto. Puede modificar toda la configuración editando el archivo `~/.why/main/why-server.toml`.

También es posible hacer que Why cree sus propios archivos de datos en el directorio actual. Para ello ejecute:

```sh
why server config
```

Lo que generará un archivo `why-server.toml` en el directorio actual, preconfigurado para usar el directorio actual para todos sus datos. Ejecutando Why desde un directorio que contiene el archivo `why-server.toml` usará los valores de ese archivo en lugar de los valores por defecto de `~/.why/main/why-server.toml`.

Durante las pruebas, ponga el binario de Why en su ruta de esta manera:

```sh
export PATH=/path/to/why/dir/target/release:$PATH
```

Donde `path/to/why/dir` es su ruta absoluta al directorio raíz de la instalación de Why.

Puede ejecutar `why` directamente (pruebe `why help` para más opciones).

## Configuración

Why se ejecuta con valores predeterminados, y puede configurarse aún más a través del archivo `why-server.toml`. Este fichero es generado por why en su primera ejecución, y contiene documentación sobre cada opción disponible.

Aunque se recomienda que realice toda la configuración de why server a través de `why-server.toml`, también es posible suministrar cambios de comandos para why que anulan cualquier configuración en el archivo.

Para obtener ayuda sobre los comandos de why y sus cambios intente:

```sh
why help
why server --help
why client --help
```

## Docker

```sh
docker build -t why -f etc/Dockerfile .
```

Puede ubicar la caché de Why para que se ejecute dentro del contenedor

```sh
docker run -it -d -v $HOME/.why:/root/.why why
```
## Compilación multiplataforma

Rust (cargo) puede compilar Why para muchas plataformas, así que en teoría ejecutar `why` como un nodo de validación en un dispositivo de baja potencia podría ser posible. Para hacer una compilación cruzada `why` en una plataforma x86 Linux y generar binarios de ARM, por ejemplo para Raspberry-pi.

## Usando Why

La página de la wiki [Cómo usar why](https://github.com/mimblewimble/docs/wiki/How-to-use-why) y las páginas de enlaces tienen más información sobre las características que disponemos, resolución de problemas, etc.

## Minando en Why

Tenga en cuenta que todas las funciones de minería de Why se han trasladado a un paquete independiente llamado [why_minner](https://github.com/mimblewimble/why-miner). Una vez que el nodo de why esté listo y funcionando, puede empezar a minar compilando y ejecutando why-miner con su nodo Why en funcionamiento.
