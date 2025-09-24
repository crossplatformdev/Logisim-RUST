[![Logisim-evolution](docs/img/logisim-evolution-logo.png)](https://github.com/logisim-evolution/logisim-evolution)

---

# Logisim-Evolution (Java Implementation)

This directory contains the Java implementation of Logisim-Evolution, a digital logic simulator.

* **Table of contents**
  * [Features](#features)
  * [Requirements](#requirements)
  * [Quick Start](#quick-start)
  * [Pictures of Logisim-evolution](docs/pics.md)
  * [More Information](docs/docs.md)
  * [For developers](docs/developers.md)
  * [How to contribute](docs/developers.md#how-to-contribute)
  * [Credits](docs/credits.md)

---

## Features

`Logisim-Evolution` is educational software for designing and simulating digital logic circuits.
`Logisim-Evolution` is [free](#license), [open-source](https://github.com/logisim-evolution), and [cross-platform](#requirements).

Project highlights:

* easy to use circuit designer,
* logic circuit simulations,
* chronogram (to see the evolution of signals in your circuit),
* electronic board integration (schematics can be simulated on real hardware),
* VHDL components (components behavior can be specified in VHDL!),
* TCL/TK console (interfaces between the circuit and the user),
* huge library of components (LEDs, TTLs, switches, SoCs),
* allows for custom libraries to be [loaded on startup](docs/automatic_library_import.md)
* supports [multiple languages](docs/docs.md#translations),
* and more!

[![Logisim-evolution](docs/img/logisim-evolution-01-small.png)](docs/pics.md)
[![Logisim-evolution](docs/img/logisim-evolution-02-small.png)](docs/pics.md)
[![Logisim-evolution](docs/img/logisim-evolution-03-small.png)](docs/pics.md)

---

## Requirements

`Logisim-Evolution` is a Java application; therefore, it can run on any operating system supporting the Java runtime environment.
It requires [Java 21 (or newer)](https://adoptium.net/temurin/releases/).

## Quick Start

### Building

```bash
# Clone and navigate to Java implementation
git clone https://github.com/crossplatformdev/Logisim-RUST.git
cd Logisim-RUST/Logisim-Evolution

# Build
./gradlew build

# Run
./gradlew run
```

## Documentation

See the [docs/](docs/) directory for comprehensive documentation including:
- [Developer Guide](docs/developers.md)
- [Documentation Overview](docs/docs.md)
- [Image Gallery](docs/pics.md)

## License

This project is licensed under GPL-3.0. See [LICENSE.md](./LICENSE.md) for details.