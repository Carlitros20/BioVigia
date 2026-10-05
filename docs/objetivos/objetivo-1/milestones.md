# Milestones
Definen qué quiero entregar, no cómo entregarlo. Un milestone no es "he terminado tal HU": marca el nivel de profundidad con el que se resuelve el problema en ese momento.

## Milestone 0: Modelo inicial de dominio

### Objetivo
Representar de forma coherente los conceptos mínimos del dominio necesarios, siguiendo una metodología explícita (DDD) que permita comprobar que el modelo se deriva de las historias de usuario y que sirve después para programar la lógica de negocio.

### Qué se entrega
- Las entidades y objetos valor mínimos, obtenidos mediante un proceso trazable y modelados en código. El proceso es: (1) extraer el lenguaje del dominio de las HUs y jornadas en un glosario; (2) anotar un caso real (una observación de iNaturalist/GBIF) marcando sustantivos y verbos; (3) identificar los eventos que ocurren en cada HU; (4) traducir el resultado a código, organizado según las buenas prácticas del lenguaje elegido. Cada concepto del código debe poder rastrearse hasta la HU de la que sale.
- Sin lógica de negocio todavía: el objetivo es tener una estructura de datos correcta. Se comprueba con una prueba que construye con el modelo el caso real anotado, sin perder ningún dato que necesiten las HUs.
- Una definición del diseño del dominio (DDD) que indique claramente qué es objeto valor, qué es entidad y qué relaciones hay entre ellos. Cada concepto se clasifica aplicando un criterio explícito (entidad: tiene identidad y ciclo de vida; objeto valor: se define por sus atributos y es inmutable), con la justificación escrita. Las relaciones y cardinalidades se recogen en un diagrama.
- Documentación breve de las decisiones de modelado: qué se decidió en cada paso del proceso, con las alternativas descartadas y la HU que lo motivó.


## Milestone 1: Primera flujo funcional verificable

### Objetivo
Demostrar que el modelo de dominio del milestone previo puede utilizarse para resolver de principio a fin un caso de uso simple relacionado con las historias de usuario.

### Qué se entrega
- La lógica mínima necesaria para ejecutar un caso de uso completo.
- Pruebas automatizadas del comportamiento principal.
