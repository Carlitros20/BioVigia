//! Estado de presencia del depredador en un tramo (#14).

/// Resultado de consultar si un depredador está presente en un tramo de río.
///
/// Sustituye a un booleano "hay depredador": que no haya avistamientos no
/// significa que no haya depredador, solo que no hay registros recientes. En el
/// M0 solo se define el tipo; cómo se calcula queda para la lógica de negocio.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EstadoPresencia {
    PresenciaDetectada,
    SinRegistrosRecientes,
}
