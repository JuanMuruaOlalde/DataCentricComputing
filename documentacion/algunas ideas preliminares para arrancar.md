# Ideas preliminares


## Posibles nombres para el proyecto

- SIC (Standard Isolable Computer)

- SIA (Standard Isolable Architecture)

- DCM (Data Centric Machine)

- Data  
  Centric  
  Computing  (DCC)


## Ideas clave relacionadas con la seguridad

### Cosas que no funcionan

- Seguridad por inconveniencia
  
  Pensar que el atacante va a abandonar por la cantidad de obstaculos que se pongan. Especialmente si esos obstaculos son muy visibles.
  (Cuanto más visibles/previsibles sean, mejor se podrá preparar para superarlos.)

- Seguridad por obscuridad

  Pensar que el atacante no va a poder atacar aquello que no conoce o que no comprende.
  (Si le interesa lo suficiente, ya se preocupará de obtener el conocimiento requerido.)

- Seguridad por identidad

  Pensar que algo es seguro porque conocemos su procedencia y confiamos en ella.
  (Toda identidad puede llegar a ser suplantantada.)

- Seguridad por vigilancia

  Pensar que el atacante no va a intentarlo si tiene probabilidad de ser capturado en el intento.
  (Los humanos somos muy malos prestando atención durante largos periodos de tiempo. Las máquinas pueden tener momentos de fallo. Es decir solo hay que esperar/propiciar un despiste en la vigilancia.)

### Cosas que si funcionan

- Principio de mínima confianza

  Proporcionar a cada parte la información y la autoridad justas para llevar a cabo cada tarea que tenga encomendada. Ni más, ni menos. En caso de duda, pecar un poco de más (ya que un poco de menos no permite hacer la tarea.)
  Aquí se aplica también lo de usar listas de cosas permitidas en lugar de listas de cosas prohibidas. Lo que no está expresamente permitido, está prohibido.

- Compartimentalización (robusta división de tareas)

  Ninguna parte puede tener más de una autoridad/información simultáneamente. En cada momento/tarea, solo ha de tener la autoridad/información que necesita. Así se evita que abuse de una autoridad/información concedida para una tarea en la realización de otra.

- Defensa en profundidad

  Tener previstas unas cuantas capas de protección para cada cosa protegida. Asi, si una capa falla, otra se encarga de contener el peligro. Cuantas más capas sean factibles, mejor.

- Simpleza y claridad

  Mejor algo pasivo (siempre está ahí), que algo activo (necesita un esfuerzo para estar ahí).

  Mejor un solo procedimiento automátizado y sencillo de comprender/recordar. Que un montón de procedimientos manuales siguiendo una lista, (y peor aún si no se comprende el porqué de alguno de ellos).

- Seguridad integrada

  Pensada desde el principio junto con la funcionalidad/uso del sistema a proteger. Si la seguridad entorpece demasiado la funcionalidad, acabará siendo inútil porque los propios usuarios la torpedearan en pos de un uso más fácil del sistema.


## Ideas y reflexiones acerca de mecanismos para compartimentalizar y aislar unas partes de otras

### Aislamiento entre programas ejecutándose en el sistema

Cualquier mecanismo de comunicación software que comparta buses hardware, puede  basarse en criptografia para garantizar que ningún tercero malicioso:
- Esté escuchando el bus compartido cuando no debe. 
- Intente alterar comunicaciones que no le corresponden. 

Pero, toda criptografia es vulnerable a un criptoanálisis suficientemente potente.

La única forma fiable de aislamiento es el propio hardware. No se puede espiar lo que no se puede escuchar. Pero para ello, la parte de control tendria que contar con un bus dedicado para cada programa. Lo que implica:

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

