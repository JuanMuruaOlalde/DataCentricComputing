# Data Centric Computing

Un sistema informático donde los datos son lo importante. Y están totalmente separados de los programas con los que se manejan. Es decir, se contemplan dos tipos diferentes de contenidos:

- Datos (documentos, videos, audios,...) No pueden contener nada que ejecute instrucciones ni altere ningún dato en la máquina.

- Programas: Instrucciones que manejan los datos y permiten su visualización, edición, transmisión, almacenamiento,...

Ambos tipos van cada uno en su parte separada del sistema. Con aislamiento físico entre partes. Para que en ningún momento pueda existir la más mínima duda de si algo es dato o programa. Los datos se tratan de un modo y los programas de otro, los datos en unos circuitos y los programas en otros.

La idea es poder trabajar y compartir información sin miedo a virus ni trojanos ni otros programas maliciosos.

Los datos pueden circular libremente de un equipo a otro. (Con las correspondientes medidas de seguridad para proteger datos confidenciales, claro está.)

Los programas son residentes en cada equipo. Y habrá dos tipos de equipos:

- Domésticos: para quien prefiera la comodidad de permitir actualizaciones automáticas de los programas en su equipo.

- Profesionales: donde se requerirá de una intervención hardware para actualizar los programas. (Como mínimo, alguien deberá girar una llave o pulsar un botón físico para autorizar cada cambio en los programas residentes. Idealmente, los programas irán en cartuchos de solo lectura que se deben extraer o conectar físicamente.)


## Nombre para el proyecto

- SIC (Standard Isolable Computer)

- SIA (Standard Isolable Architecture)

- DCM (Data Centric Machine)

- Data
  Centric
  Computing

## Especificaciones principales

- Separación total, por hardware, entre la parte ejecutable y la parte de datos. Ningún programa puede escribir en la parte ejecutable, solo en la parte de datos. Dos buses físicos completamente separados. Dos memorias físicas completamente separadas. Dos almacenamientos completamente separados.

- Una tercera parte de control y monitorización. Separada totalmente, por hardware, de la parte ejecutable y de la de datos. Las tres partes solo comparten un tercer bus de control o de sincronización o similar. 

- Obviamente, los usuarios solo pueden intercambiarse datos en su trabajo habitual. No pueden haber nada ejecutable mezclado con los datos. Todo lo que sea ejecutable ha de quedar claro que lo es.

- Para modificar algo en la parte ejecutable (por ejemplo instalar o actualizar un programa). El usuario ha de involucrarse activamente (por ejemplo, girando una llave en la parte de control e indicando en la consola lo que quiere instalar o actualizar).

- Cada programa solo puede utilizar su propia partición de la memoria de ejecución y su propia partición de la memoria de datos. Las memorias físicas están gestionadas por la parte de control.

- Cada programa solo puede utilizar sus propias ventanas dentro de los periféricos de visualización. Los periféricos físicos están gestionados por la parte de control.

- Cada programa solo puede leer/escribir en su propia partición en la parte de datos. Para pasar datos de un programa a otro, el usuario ha de involucrarse activamente (por ejemplo, copiando o moviendo archivos de una partición a otra)

- Cada programa solo puede utilizar sus propios canales dentro de los periféricos de comunicaciones. Los periféricos físicos están gestionados por la parte de control.

## Ideas de implementación

- Cuando un programa necesita un recurso (memoria, almacenamiento, visualización, comunicaciones,...), ha de pedir a la parte de control que le suministre las particiones correspondientes para utilizar los recursos que necesite. Como dichos recursos están manejados por la parte de control, el programa envia/recibe peticiones a/de los recursos a través de esta. La parte de control se encarga de implementar el aislamiento estricto entre programas.

- El almacenamiento de la parte ejecutable permanecerá en modo solo-lectura durante el funcionamiento habitual de la máquina. Solo se habilitará escritura cuando se necesite instalar o actualizar un programa.

- Cualquier actualización en la parte de control y monitorización. Implica una actualización hardware.


## Mecanismos para aislar unas partes de otras

### Enlaces unidirecionales

Por ejemplo, un enlace unidireccional óptico es aquel donde una parte solo tiene un emisor y la otra solo un receptor. El contenido pueden pasar en un sentido, pero no en otro.

Es útil por ejemplo para aislar secciones de archivo:

- El contenido se introduce al archivo en soportes físicos de solo lectura. La única forma de alterar contenidos es mediante un acceso físico directo al almacén donde se almacenan dichos soportes.

- Desde el exterior solo se pueden dar órdenes de mover ciertos soportes a un dispositivo lector. Copia del contenido se transmite al exterior a través de enlaces unidirecionales.
