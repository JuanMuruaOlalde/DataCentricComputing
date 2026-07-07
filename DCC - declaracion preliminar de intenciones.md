# DCC (Data Centric Computing)

## Declaración de intenciones

Se trata de diseñar y construir un sistema informático donde los datos son lo importante. Estando totalmente separados de los programas ejecutables que los manejan. Es decir, se contempla diferenciar completamente dos tipos de contenidos:

- DATOS (documentos, videos, audios,...) Son solo información, no pueden contener nada que ejecute instrucciones ni altere ningún otro dato en la máquina.

- PROGRAMAS: Son las instrucciones que manejan los datos y permiten su visualización, edición, transmisión, almacenamiento,...

Ambos tipos de contenido han de tratarse cada uno en su parte separada del sistema. Existiendo un aislamiento completo (físico) entre ambas partes. Para que en ningún momento pueda existir la más mínima duda de si algo es dato o programa. Los datos se tratan de un modo y los programas de otro, los datos circulan por unos circuitos y los programas por otros.

Además, en la parte de programas ha de garantizarse que estos no se alteran de ningún modo durante el funcionamiento normal del sistema. Una vez instalado un programa en el sistema, solo podrá actualizarse teniendo acceso físico y realizando un esfuerzo consciente deliberado por parte de quien realiza las actualizaciones.

Sin embargo, los datos podrán circular libremente, incluso de un equipo a otro. (Con las correspondientes medidas de seguridad para proteger datos confidenciales, claro está.)

Obviamente, solo pueden intercambiarse datos (información). No puede haber nada ejecutable (ni susceptible de ser convertido a ejecutable) mezclado con los datos.

La idea es poder trabajar y compartir información sin miedo a virus ni trojanos ni otros programas maliciosos.


## Especificaciones básicas

### Funcionamiento

- La separación entre la parte ejecutable y la parte de datos será total, garantizada por hardware. Dos buses físicos completamente separados. Dos memorias físicas completamente separadas. Dos almacenamientos completamente separados.

- En todo momento se ha de  garantizar que ningún programa puede escribir en la parte ejecutable, solo en la parte de datos. Y, además solo en los datos que maneja; no pudiendo interferir en los datos manejados por otro programa.

- La gestión del sistema corre a cargo de una tercera parte de control/arbitraje. Separada totalmente, por hardware, de la parte ejecutable y de la parte de datos. Las tres partes solo comparten un tercer bus de control o de sincronización o similar. 

- Cada programa solo puede utilizar su propia partición de la memoria de ejecución y su propia partición de la memoria de datos. Las memorias físicas están gestionadas por la parte de control/arbitraje.

- Cada programa solo puede utilizar sus propias ventanas dentro de los periféricos de visualización. Los periféricos físicos están gestionados por la parte de control.

- Cada programa solo puede leer/escribir en su propia partición en la parte de datos. Para pasar datos de un programa a otro, el usuario ha de involucrarse activamente (por ejemplo, permitiendo la copia o movimiento de archivos de una partición a otra) (por ejemplo, copiando datos en una ventana de un programa y pegándolos en una de otro programa).

- Cada programa solo puede utilizar sus propios canales dentro de los periféricos de comunicaciones. Los periféricos físicos están gestionados por la parte de control/arbitraje.

- Cuando un programa necesita un recurso (memoria, almacenamiento, visualización, comunicaciones,...), ha de pedir a la parte de control/arbitraje que le suministre las particiones correspondientes para utilizar los recursos que necesite. Como dichos recursos están manejados por la parte de control/arbitraje, el programa envia/recibe peticiones a/de los recursos a través de esta. La parte de control/arbitraje se encarga de implementar un aislamiento estricto entre programas.



### Actualizaciones

- Cada programa ejecutable ha de tener algún modo de verificar la autenticidad de las actualizaciones que reciba. Antes de dejarse sobreescribir, ha de poder probar que los cambios provienen de una fuente conocida de confianza. De una fuente que ya era conocida por el programa antes de la actualización.

- Se han de contemplar dos tipos de equipos:

  - Domésticos: Con algún módulo adicional que permita actualizaciones remotas de los programas en el equipo. Pero requiriendo intervención expresa por parte del usuario para realizarlas (por ejemplo girar una llave o pulsar un botón físico para poner en modo escritura el almacenamiento de los programas residentes).

  - Profesionales: Donde se requerirá acceso físico expreso para actualizar los programas. Por ejemplo, requerir cambiar físicamente el dispositivo de almacenamiento de los programas residentes; o requerir hardware especializado para actualizar el contenido de ese dispositivo.

- ¿Podria haber una tercera vía: equipos como los domésticos pero en los que la comunicación con el módulo adicional requiera otra conexión de red distinta de la de datos?

