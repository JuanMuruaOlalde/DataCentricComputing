# Premisas base

Tres dominios separados:
- Ejecución de programas de usuario -> EP
- Manipulación de datos/información de usuario -> MD
- Gestión/Control/Monitorización del funcionamiento del sistema -> GCM

Cada dominio está aislado a nivel hardware. En la medida de lo posible, cuenta con sus propios recursos (procesador, ALU, pipelines, buses, memoria, persistencia,...) dedicados en exclusiva.

Entre dominios comparten tan solo el hardware mínimo imprescindible. El justo y necesario requerido para materializar el modelo de comunicación que se establezca para intercambiar entre ellos la información operativa mínima necesaria para funcionar.

Cada recurso o periférico presente en el sistema será de uso exclusivo:
- o bien para programas ejecutables.
- o bien para datos de usuario.
Sin posibilidad de mezclar ambos aspectos en ningún momento.

Los subsistemas tipo EP no tienen acceso a ningún periférico de comunicación con el exterior al sistema.

Solo los subsistemas tipo MD pueden recibir/enviar datos desde/al exterior. Es decir, solo datos de usuario entran o salen del sistema.


# Subsistemas de trabajo

Lo ideal es que cada programa de aplicación se ejecute en un subsistema EP/MD dedicado, separado de los EP/MD de los demás programas de aplicación que puedan estar ejecutándose en ese momento en el sistema. Los intercambios de datos entre ellos vendrán gestionados y controlados por el subsistema GCM.


## Subsistema "tipo EP", `ejecutar programas`

Se refiere a programas de aplicación que maneja el usuario. Al software productivo que utiliza el usuario en su trabajo.

Este tipo de subsistema ejecuta la lógica de operación requerida para obtener los resultados deseados a partir de los datos que se van a manipular.

El subsistema tipo EP constará de dos partes diferenciadas:
- Una *quasi-read-only*, con una copia del código ejecutable del programa que se esté ejecutando en ese momento.
- Otra *read/write*, la RAM usada como espacio de trabajo para guardar la información dinámica que necesite ese programa para su flujo de control de ejecución interno.

Nada de datos de usuario en este tipo de subsistema.


## Subsistema "tipo MD", `manipular datos`

Se refiere a datos de usuario. A la información útil que se manipula en el sistema; bien sean datos de partida o bien sean resultados intermedios o finales.

Este tipo de subsistema mueve/copia datos de usuario y realiza operaciones aritmético/lógicas sobre ellos. Según se lo indique el subsistema EP vinculado a él. La comunicación entre ambos subsistemas, EP y MD, se realizará con dos canales unidireccionales dedicados: uno EP -> MD y otro MD -> EP.

El subsistema tipo MD constará de tres partes diferenciadas:
- Una *read-only*, la ROM con el firmware que lo gobierna.
- Otra *read/write*, la RAM usada como espacio de trabajo para guardar la información dinámica que necesite el firmware para su flujo de control de ejecución interno.
- Otra *read/write*, la RAM y la persistencia usada como espacio de trabajo para las manipulaciones de datos de usuario.

Nada de código ejecutable en este tipo de subsistema.



# Subsistema GCM, `gestión/control/monitorización`

Este tipo de subsistema es único en el sistema. O, a lo sumo, está replicado por redundancia para garantizar la continuidad de funcionamiento.

El subsistema tipo GCM constará de tres partes diferenciadas:
- Una *read-only*, la ROM con el firmware que lo gobierna.
- Otra *read/write*, la RAM usada como espacio de trabajo para guardar la información dinámica que necesite el firmware para su flujo de control de ejecución interno.
- Otra *read/write*, la RAM usada para intercambios de datos de usuario entre subsistemas MD.

Se encarga de tareas tales como:
- Mantener informado al usuario de lo que está pasando en el sistema.
- Arrancar programas según lo solicite el usuario.
- Coordinar el acceso a los recursos y periféricos por parte de los subsistemas tipo EP. 
- Coordinar el acceso a los recursos y periféricos por parte de los subsistemas tipo MD.
 
Prestará especial cuidado para garantizar que un determinado programa de aplicación ejecutándose en un subsistema EP/MD no pueda interferir con ningún otro programa ejecutándose en otro subsistema EP/MD. El acceso a recursos y periféricos comunes estará estrictamente segmentado; cada EP/MD (cada programa) solo accede a la porción que se le asigne.

Se encargará también de las transferencias de datos entre subsistemas MD cuando el usuario así lo solicite. (Por ejemplo, copiar/pegar datos del espacio manejado por un programa al manejado por otro.)

Nada de datos de usuario en este tipo de subsistema. Al igual que en los subsistemas EP, toda manipulación de datos se hará vía un subsistema tipo MD auxiliar.


# notas

Sé que va a salir muy caro multiplicar todo el hardware (por ejemplo, RAM, buses, registros,...) en cada subsistema. Pero considero que es la única manera de asegurar la no alteración maliciosa de la forma de trabajo del sistema en su conjunto. De asegurar de que cada programa hace siempre su trabajo, sin posibilidad de ataques ni de interferencias externas ajenas.

También sé que es muy difícil proteger un entorno de sí mismo. Por ello asumo que toda aquella persona con acceso físico al hardware es de confianza. A efectos de diseñar la arquitectura, me preocupa solo la alteración maliciosa llevada a cabo por terceras personas aprovechando las debilidades del sistema (debilidades que incluyen también la inconsciencia y falta de cuidado de las personas con acceso legítimo al sistema).
