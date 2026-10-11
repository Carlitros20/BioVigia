# Historias de Usuario
Van con el tag 'user-stories' y cada una se irá resolviendo mediante issues en distintos milestones, según el nivel de abstracción de cada fase.

## [HU001] No sé si mi especie objetivo está siendo depredada en el río al que voy a ir
Como pescador que va a pasar una jornada completa en un río (8-12 horas en el mismo sitio) que ya conozco de Toledo, no tengo forma de saber antes de salir de casa si hay especies invasoras que depreden a la especie que busco, por lo que a veces pierdo el día entero yendo a una zona con pocas o nulas posibilidades de captura.

**Jornada relacionada:** Jornada 1 (Antonio).

### El problema
Intervienen estos conceptos:
- **Especie objetivo**: la que el pescador quiere capturar (ej. cangrejo).
- **Especie invasora depredadora**: la que se la come o la desplaza (ej. visón americano).
- **Zona**: tramo de río donde pretende pescar.
- **Avistamiento**: observación de una especie en un lugar y una fecha concretos, aportada por alguien.
- **Ventana temporal**: solo cuentan los avistamientos recientes (ej. últimos 15 días).

Cadena causal: avistamientos recientes del depredador en la zona → menor probabilidad de capturar la especie objetivo → el pescador pierde la jornada, el combustible y el tiempo.

Limitación importante: la ausencia de avistamientos no garantiza la ausencia del depredador. Solo significa que nadie lo ha reportado.

### Ejemplo concreto
Antonio quiere pescar cangrejos en un tramo de río cerca de Toledo. Consulta la zona y hay 3 avistamientos de visón americano en los últimos 15 días. La zona se considera con depredador presente y Antonio decide cambiar de ubicación. En un segundo tramo no hay avistamientos recientes y lo elige como alternativa.

### Datos: de dónde se obtienen
- iNaturalist (API de observaciones): https://api.inaturalist.org/v1/docs/
- GBIF (API de ocurrencias): https://techdocs.gbif.org/en/openapi/

Campos necesarios de cada avistamiento: especie, fecha de observación, latitud, longitud, fuente e identificador de la observación original.

Ejemplo de consulta real a iNaturalist (visón en un recuadro alrededor de Toledo):
https://api.inaturalist.org/v1/observations?taxon_name=Neogale%20vison&swlat=39.6&swlng=-4.4&nelat=40.0&nelng=-3.7&quality_grade=research&per_page=5

Ejemplo equivalente en GBIF:
https://api.gbif.org/v1/occurrence/search?scientificName=Neogale%20vison&country=ES&hasCoordinate=true&limit=5

### Criterio de aceptación (ejemplo)
Dada una zona y una especie depredadora, cuando se consultan los avistamientos de los últimos 15 días, entonces se indica si hay avistamientos y cuántos.


## [HU002] No sé cómo se está expandiendo una especie invasora a lo largo del tiempo
Como gestora ambiental, no tengo forma de ver cómo evoluciona la presencia de una especie invasora en una zona a lo largo del tiempo (primer y último avistamiento, si ha desaparecido de un río o ha aparecido en uno nuevo), por lo que no puedo priorizar dónde actuar con los recursos que tengo.

**Jornada relacionada:** Jornada 2 (Tabitha).

### El problema
Intervienen estos conceptos:
- **Especie invasora** y **río/zona** de estudio.
- **Avistamiento**, con fecha y lugar.
- **Serie temporal**: los avistamientos de una especie en una zona ordenados en el tiempo.
- **Primer y último avistamiento** en cada zona.
- **Estado de presencia**: persiste, ha desaparecido o es nueva en esa zona.

Cadena causal: los avisos dispersos impiden ver la serie temporal → no se ve si la especie avanza o retrocede → no se pueden priorizar las inspecciones.

### Ejemplo concreto (datos ilustrativos)
Tabitha selecciona el río A y el visón americano. Obtiene: primer avistamiento en 2019, último en 2022, sin avistamientos desde entonces (¿desaparecido?). En el río B, primer avistamiento en 2025 (zona nueva). Prioriza inspeccionar el río B.

### Datos: de dónde se obtienen
- GBIF (API de ocurrencias, con series históricas multianuales): https://techdocs.gbif.org/en/openapi/
- iNaturalist (API de observaciones): https://api.inaturalist.org/v1/docs/

Mismos campos que en HU001. Aquí la fecha es imprescindible para calcular primer y último avistamiento.

### Criterio de aceptación (ejemplo)
Dada una especie y un río, cuando se consultan todos sus avistamientos, entonces se devuelven la fecha del primer y del último avistamiento.