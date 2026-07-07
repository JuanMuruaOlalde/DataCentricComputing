# Premisas base

Cada subsistema está aislado a nivel hardware. En la medida de lo posible, cuenta con sus propios recursos (procesador, memoria, persistencia,...), con sus propios chips y buses, dedicados en exclusiva.

Entre subsistemas comparten tan solo el hardware mínimo imprescindible. El justo y necesario requerido para materializar los protocolos de comunicación que se establezcan para intercambiarse entre ellos la información operativa mínima necesaria para funcionar.

Cada recurso o periférico presente en el sistema será de uso exclusivo:
- o bien para programas ejecutables.
- o bien para datos de usuario.
Sin posibilidad de mezclar ambos aspectos en ningún momento.

Los subsistemas tipo EP no tienen acceso a ningún periférico de comunicación con el exterior al sistema.

Solo los subsistemas tipo MD pueden recibir/enviar datos desde/al exterior. Es decir, solo datos de usuario entran o salen del sistema.



# Subsistema "tipo EP", `ejecutar programas`

Se refiere a programas de aplicación que maneja el usuario. Al software productivo que utiliza el usuario en su trabajo.

Este tipo de subsistema constará de dos partes diferenciadas:
- Una *read-only*, con el código ejecutable del programa que está ejecutando.
- Otra *read/write*, usada como espacio de trabajo para guardar la información dinámica que necesite ese programa para su flujo de control de ejecución interno.

Nada de datos de usuario en este tipo de subsistema.


# Subsistema "tipo MD", `manipular datos`

Se refiere a datos de usuario. A la información útil que se manipula en el sistema; bien sean datos de partida o bien sean resultados intermedios o finales.

Este tipo de subsistema mueve/copia datos de usuario y realiza operaciones aritmético/lógicas sobre ellos, según se lo indique el subsistema EP vinculado a él.

Nada ejecutable ni nada susceptible de ser convertido a ejecutable en este tipo de subsistema.


# Subsistema GCM, `gestión/control/monitorización`

Este tipo de subsistema es único en el sistema. O, a lo sumo, está replicado por redundancia para garantizar la continuidad de funcionamiento.

Constará también de dos partes diferenciadas:
- Una *read-only*, con el código ejecutable del firmware que está ejecutando.
- Otra *read/write*, usada como espacio de trabajo para guardar la información dinámica que necesite ese firmware para su flujo de control de ejecución interno.

Se encarga de tareas tales como:
- Mantener informado al usuario de lo que está pasando en el sistema.
- Arrancar programas según lo solicite el usuario.
- Coordinar el acceso a los recursos y periféricos por parte de los subsistemas tipo EP. 
- Vincular cada uno de esos EP con su correspondiente subsistema MD. 
- Coordinar el acceso a los recursos y periféricos por parte de los subsistemas tipo MD.
 
Prestará especial cuidado para garantizar que un determinado programa de aplicación ejecutándose en un subsistema EP no pueda interferir con ningún otro programa ejecutándose en otro subsistema EP. El acceso a recursos y periféricos comunes estará estrictamente segmentado; cada EP (cada programa) solo accede a la porción que se le asigne.

Se encargará también de las transferencias de datos entre subsistemas MD cuando el usuario así lo solicite. (Por ejemplo, copiar/pegar datos del espacio manejado por un programa al manejado por otro.)

Nada de datos de usuario en este tipo de subsistema. Toda manipulación de datos se hará vía un subsistema tipo MD auxiliar.

