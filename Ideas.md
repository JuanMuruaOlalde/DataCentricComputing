# Data Centric Computing

## Declaración de intenciones

Se trata de diseñar y construir un sistema informático donde los datos son lo importante. Estando totalmente separados de los programas ejecutables que los manejan. Es decir, se contemplan dos tipos diferentes de contenidos:

- Datos (documentos, videos, audios,...) Son solo información, no pueden contener nada que ejecute instrucciones ni altere ningún otro dato en la máquina.

- Programas: Son las instrucciones que manejan los datos y permiten su visualización, edición, transmisión, almacenamiento,...

Ambos tipos de contenido han de tratarse cada uno en su parte separada del sistema. Existiendo un aislamiento completo (físico) entre ambas partes. Para que en ningún momento pueda existir la más mínima duda de si algo es dato o programa. Los datos se tratan de un modo y los programas de otro, los datos circulan por unos circuitos y los programas por otros.

Además, en la parte de programas ha de garantizarse que estos no se alteran de ningún modo. Una vez instalado un programa en el sistema, nada ni nadie podrá alterarlo sin conocimiento expreso de las personas que manejan el sistema.

Sin embargo, los datos podrán circular libremente, incluso de un equipo a otro. (Con las correspondientes medidas de seguridad para proteger datos confidenciales, claro está.)

Obviamente, solo pueden intercambiarse datos (información). No puede haber nada ejecutable mezclado con los datos.

La idea es poder trabajar y compartir información sin miedo a virus ni trojanos ni otros programas maliciosos.

## Nombre para el proyecto

- SIC (Standard Isolable Computer)

- SIA (Standard Isolable Architecture)

- DCM (Data Centric Machine)

- Data  
  Centric  
  Computing  

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


## Ideas y reflexiones acerca de los mecanismos para aislar unas partes de otras

### Aislamiento entre programas

Cualquier mecanismo de comunicación software que comparta buses hardware, ha de basarse en criptografia para garantizar que ningún tercero malicioso esté escuchando el bus compartido cuando no debe o intente alterar comunicaciones que no le corresponden. Toda criptografia es vulnerable a un criptoanálisis suficientemente potente.

La única forma fiable de aislamiento es el hardware. No se puede espiar lo que no se puede escuchar. Pero para ello, la parte de control tendria que contar con un bus dedicado para cada programa. Lo que implica:

- Cada programa tendria que ir en su própio módulo hardware enchufable al sistema.

- La máquina solo puede contar con un número finito de ranuras donde enchufar programas. Aunque este número se puede ir ampliando, añadiendo el correspondiente hardware.

- Cada cartucho de programa, al insertarse por primera vez en una ranura, recibe un token identificador único. Solo válido para esa ranura (ese programa). Ese token-ranura-programa queda almacenado en la parte de control.

- Toda petición de acceder a algún dispositivo ha de ir avalada por su respectivo token y provenir de la correspondiente ranura. La parte de control solo permitirá a un programa acceder a aquello que haya sido depositado/solicitado/creado con el token de ese programa.

Obviamente, quien tenga acceso físico al hardware puede acceder a lo que desee. Pero acceder físicamente a una máquina para subvertir sus cartuchos de programas o sus ranuras de conexión es difícil. Sobre todo si la máquina esta está bien protegida.

### Interface con el usuario

Por ejemplo, el espacio de pantalla. Solo la parte de control puede pintar en la pantalla. Cada programa solicita a la parte de control que le asigne una ventana y va dando instrucciones de qué pintar en esa ventana.

El cursor del ratón está manejado por la parte de control. Quien se encarga de comunicar a cada programa los eventos que se produzcan cuando el cursor está sobre alguna de las ventanas pertenecientes a ese programa.

De manera similar, el teclado está manejado por la parte de control. Las pulsaciones de teclado se envian solo al programa que estaba activo en el momento de pulsar cada tecla.

Etc.

### Almacenamiento de información

Toda petición de guardar/recuperar información en/desde algún dispositivo de almacenamiento ha de ir avalada por su respectivo token y provenir de la correspondiente ranura. La parte central no permite a ningún programa escribir/leer en el espacio de otro programa.

Toda información perteneciente a un cartucho de programa que ya no esté enchufado en la máquina. No va a poder ser accedida de nuevo. Así es que se puede eliminar del sistema.

Si deseamos conservar la información de un programa que vamos a desenchufar. Este programa deberá contar con alguna manera de realizar una copia de la información antes de ser desenchufado. Y un mecanismo para recuperar dicha copia cuando se enchufe de nuevo.

Es necesario prever algún mecanismo que permita a un cartucho enmendar una retirada erronea. (Por ejemplo, que pueda mantener su mismo token si se vuelve a introducir en la misma ranura antes de unas horas, unos dias,... Este "tiempo de gracia" podria ser negociado entre el cartucho y la parte central cuando el cartucho se introduce por primera vez.)


### Traspasos de información de un programa a otro

Cuando persona usuaria indica de enviar una cierta información desde un programa a otro (por ejemplo, arrastrando algo de una ventana a otra). El programa emisor indica a la parte de control qué información desea pasar al otro programa. El programa receptor recibe una copia de la información desde la parte de control.

Esto puede utilizarse también para enviar o recibir información a/desde un dispositivo enchufable. Obviamente este dispositivo/cartucho tendria su propio programa. En caso de desear tener esta funcionalidad en la máquina, seria conveniente prever unas ranuras enchufables fácilmente accesibles por el usuario.

### Información de solo lectura

Por ejemplo, un enlace unidireccional óptico es aquel donde una parte tiene solo un emisor y la otra solo un receptor. El contenido pueden pasar en un sentido (emisor=>receptor), pero no en otro.

Es útil por ejemplo para aislar secciones de archivo:

- El contenido se introduce al archivo en soportes físicos de solo lectura. La única forma de alterar contenidos es mediante un acceso físico directo al almacén donde se almacenan dichos soportes.

- Desde el exterior solo se pueden dar órdenes de mover cierto contenido a un dispositivo lector. Una copia de ese contenido se transmite al exterior a través de enlaces unidirecionales.
