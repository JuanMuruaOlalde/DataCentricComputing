# DCC (Data Centric Computing)

## Declaración de intenciones

Se trata de diseñar y construir un sistema informático donde los datos son lo importante. Estando totalmente separados de los programas ejecutables que los manejan. Es decir, se contempla diferenciar completamente dos tipos de contenidos:

- DATOS (documentos, videos, audios,...) Son solo información, no pueden contener nada que ejecute instrucciones ni altere ningún otro dato en la máquina.

- PROGRAMAS: Son las instrucciones que manejan los datos y permiten su visualización, edición, transmisión, almacenamiento,...

Ambos tipos de contenido han de tratarse cada uno en su parte separada del sistema. Existiendo un aislamiento completo (físico) entre ambas partes. Para que en ningún momento pueda existir la más mínima duda de si algo es dato o programa. Los datos se tratan de un modo y los programas de otro, los datos circulan por unos circuitos y los programas por otros.

Además, en la parte de programas ha de garantizarse que estos no se alteran de ningún modo. Una vez instalado un programa en el sistema, nada ni nadie podrá alterarlo sin conocimiento expreso de las personas que manejan el sistema.

Sin embargo, los datos podrán circular libremente, incluso de un equipo a otro. (Con las correspondientes medidas de seguridad para proteger datos confidenciales, claro está.)

Obviamente, solo pueden intercambiarse datos (información). No puede haber nada ejecutable mezclado con los datos.

La idea es poder trabajar y compartir información sin miedo a virus ni trojanos ni otros programas maliciosos.


## Especificaciones básicas

- Se han de contemplar dos tipos de equipos:

  - Domésticos: para quien prefiera la comodidad de permitir actualizaciones automáticas de los programas en su equipo.

  - Profesionales: donde se requerirá de una intervención clara y expresa para actualizar los programas. (Por ejemplo, como mínimo, alguien debería girar una llave o pulsar un botón físico para poner en modo escritura el almacenamiento de los programas residentes. Lo ideal seria que los programas estuvieran en cartuchos físicos de solo lectura que se conectaran o extrajeran manualmente.)

- La separación entre la parte ejecutable y la parte de datos será total, garantizada por hardware. Dos buses físicos completamente separados. Dos memorias físicas completamente separadas. Dos almacenamientos completamente separados. 

- En todo momento se ha de  garantizar que ningún programa puede escribir en la parte ejecutable, solo en la parte de datos. Y, además solo en los datos que maneja; no pudiendo interferir en los datos manejados por otro programa.

- La gestión del sistema corre a cargo de una tercera parte de control y monitorización. Separada totalmente, por hardware, de la parte ejecutable y de la parte de datos. Las tres partes solo comparten un tercer bus de control o de sincronización o similar; bus manejado por la parte gestora. 

- Cada programa solo puede utilizar su propia partición de la memoria de ejecución y su propia partición de la memoria de datos. Las memorias físicas están gestionadas por la parte de control.

- Cada programa solo puede utilizar sus propias ventanas dentro de los periféricos de visualización. Los periféricos físicos están gestionados por la parte de control.

- Cada programa solo puede leer/escribir en su propia partición en la parte de datos. Para pasar datos de un programa a otro, el usuario ha de involucrarse activamente (por ejemplo, copiando o moviendo archivos de una partición a otra) (por ejemplo, copiando datos en un programa y pegándolos en otro)

- Cada programa solo puede utilizar sus propios canales dentro de los periféricos de comunicaciones. Los periféricos físicos están gestionados por la parte de control.

- Cuando un programa necesita un recurso (memoria, almacenamiento, visualización, comunicaciones,...), ha de pedir a la parte de control que le suministre las particiones correspondientes para utilizar los recursos que necesite. Como dichos recursos están manejados por la parte de control, el programa envia/recibe peticiones a/de los recursos a través de esta. La parte de control se encarga de implementar un aislamiento estricto entre programas.

- El almacenamiento de la parte ejecutable permanecerá en modo solo-lectura durante el funcionamiento habitual de la máquina. La escritura no será posible más que cuando se necesite instalar o actualizar un programa. Y siempre se deberá activar expresamente por parte de la persona usuaria del sistema. A poder ser requiriendo algún medio físico para hacerlo (es decir, algo que no se pueda hacer "sin saberlo" en un momento "de despiste").

- Cada programa ha de tener el modo de verificar la autenticidad de las actualizaciones que reciba. Ha de poder probar que provienen de una fuente conocida de confianza. De una fuente que ya era conocida por el programa antes de la actualización.

- La única manera de añadir nuevas fuentes conocidas de confianza a un programa será si provienen de una fuente de confianza ya conocida previamente por dicho programa. O si son introducidas expresamente por la persona usuaria del sistema. A poder ser requiriendo algún medio físico para hacerlo (es decir, algo que no se pueda hacer "sin saberlo" en un momento "de despiste").

- Cualquier actualización en la parte de control y monitorización. Implica una actualización hardware, cambiar físicamente circuitos. A poder ser, ha de ser una actualización compleja de realizar (es decir, algo que no se pueda hacer "discretamente" aprovechando un momento en que las personas que manejan el equipo no se den cuenta.).

