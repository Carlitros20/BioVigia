#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EstadoPresencia {
    PresenciaDetectada,
    SinRegistrosRecientes,
}
