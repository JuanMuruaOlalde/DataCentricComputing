# Temas que ha de atender un Sistema Operativo

- Drivers para los dispositivos hardware.

- Gestión de la memoria (Virtual Memory)

- Gestión de la ejecución (Virtual CPU|GPU)
  - Scheduler (tiempo)
  - Dispatcher (recursos)

- Gestión del almacenamiento (Virtual File System)
  - Versioning File System (con control de cambios)
  - Journaling File System (con regisro-log- de cambios)

- Comunicación entre procesos y entre aplicaciones dentro de la propia máquina
  - IPC (Inter-Process Communication)
  - Pipes
  - Shared Memory
  - Sockets

- Control de autentificación y autorización (QUIEN puede hacer QUÉ)

- Gestión de los comandos e información desde el usuario (Virtual InputInterfaces)
  - Teclado
  - Ratón
  - Microfono

- Gestión de la presentación hacia el usuario (Virtual OutputInterfaces)
  - Pantallas
  - Sonidos

- Comunicación con otras máquinas (networking)

- API para acceder a todos los servicios citados.

