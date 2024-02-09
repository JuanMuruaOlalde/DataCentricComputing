# Temas que ha de atender un Sistema Operativo

Las dos principales tareas que ha de desempeñar un sistema operativo son:

- Facilitar la programación de software (acceso a distintos hardwares, estandarización de servicios comunes,...)

- Regular/coordinar el trabajo de los distintos procesos software que puedan estar corriendo en un sistema en un momento dado.

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

## El proceso de arranque

Desde que se enciende el hardware, hasta que el sistema operativo pueda hacerse cargo del mismo. Algo ha de cargar y ejecutar el sistema operativo. Esta es la tarea del firmware y del proceso de bootstrap.

