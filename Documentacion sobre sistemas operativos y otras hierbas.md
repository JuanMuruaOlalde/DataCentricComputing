# DCC (Data Centric Computing), documentación para aprender

## Seguridad

[Actor Model](https://en.wikipedia.org/wiki/Actor_model)

[Crockford on JavaScript - Part 5: The End of All Things](https://www.youtube.com/watch?v=47Ceot8yqeI)

[The Lazy Programmer's Guide to Secure Computing](https://www.youtube.com/watch?v=eL5o4PFuxTY)

[Arm Morello Program](https://www.arm.com/architecture/cpu/morello)

[CHERI - The Digital Security by Design (DSbD) Initiative](https://www.cl.cam.ac.uk/research/security/ctsrd/cheri/dsbd.html)


## Temas que ha de atender un Sistema Operativo

Las dos principales tareas que ha de desempeñar un sistema operativo son:

- Facilitar la programación de software, proporcionando un entorno estandarizado y estable. Con una serie de servicios comunes, independientes del hardware.

- Regular/coordinar el trabajo de los distintos programas que puedan estar corriendo en el sistema en un momento dado.

Para ello se ha de ocupar de aspectos tales como:

- Drivers para los dispositivos hardware.

- Gestión de la memoria (Virtual Memory)

- Gestión de la ejecución (Virtual CPU|GPU)
  - Scheduler (tiempo)
  - Dispatcher (recursos)

- Gestión del almacenamiento (Virtual File System)
  - Versioning File System (con control de cambios)
  - Journaling File System (con registro-log- de cambios)

- Comunicación entre procesos y entre aplicaciones dentro de la propia máquina
  - IPC (Inter-Process Communication)
  - Pipes
  - Shared Memory
  - Sockets
  - ...

- Comunicación con otras máquinas (networking)

- Control de autentificación y autorización (QUIEN puede hacer QUÉ)

- Gestión de los comandos e información desde el usuario (Virtual InputInterfaces)
  - Teclado
  - Ratón
  - Microfono
  - ...

- Gestión de la realimentación e información hacia el usuario (Virtual OutputInterfaces)
  - Pantallas
  - Sonidos
  - ...

- API (Application Programming Interfaces) para acceder a todos los servicios citados.

### El proceso de arranque

Desde que se enciende el hardware, hasta que el sistema operativo pueda hacerse cargo del mismo. Algo ha de cargar y ejecutar ese sistema operativo. Esta es la tarea del *firmware* y del proceso de *bootstrap*.


### algunos enlaces a documentación

[OS Dev wiki - Beginner Mistakes](https://wiki.osdev.org/Beginner_Mistakes)

[OS Dev wiki - Getting Started](https://wiki.osdev.org/Getting_Started)

[OS Dev wiki - Bare Bones Development](https://wiki.osdev.org/Bare_Bones)

[OS Dev wiki - Executable Formats](https://wiki.osdev.org/Category:Executable_Formats)

[OS Dev wiki - OS theory](https://wiki.osdev.org/Category:OS_theory)

[OS Dev wiki - a GCC cross-compiler for your own operating system](https://wiki.osdev.org/GCC_Cross-Compiler)

[OS Dev wiki - Rust](https://wiki.osdev.org/Rust)

[TRON operating system project](https://www.tron.org/)

[Raspberry Pi - Linux Kernel source - github](https://github.com/raspberrypi/linux)

[The Linux Kernel documentation](https://www.kernel.org/doc/html/latest/)

[Writing an OS in Rust - Philipp Oppermann's blog](https://os.phil-opp.com/)

## Hardware: arquitecturas de microprocesadores y sistemas

[Harvard architecture](https://en.wikipedia.org/wiki/Harvard_architecture)

[Modified Harvard architecture](https://en.wikipedia.org/wiki/Modified_Harvard_architecture)

[Memory Tagged architecture](https://en.wikipedia.org/wiki/Tagged_architecture)

[Executable-space protection](https://en.wikipedia.org/wiki/Executable-space_protection)

[HDL - Hardware Description Language](https://en.wikipedia.org/wiki/Hardware_description_language)

[Verilog - an hardware description language](https://en.wikipedia.org/wiki/Verilog)

[VHDL - an hardware description language](https://en.wikipedia.org/wiki/VHDL)

[arm - Learn the Architecture - Guides](https://www.arm.com/architecture/learn-the-architecture)

[Linaro empowers rapid product deployment within the dynamic Arm ecosystem](https://www.linaro.org/)

[AVR® DA Product Family](https://www.microchip.com/en-us/products/microcontrollers-and-microprocessors/8-bit-mcus/avr-mcus/avr-da)

[Arm Cortex‑A710 Core Technical Reference Manual](https://developer.arm.com/documentation/101800/0201/Technical-overview/Core-components)


 
## Algunos enlaces a documentación variada

