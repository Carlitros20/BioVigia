//! Construye con el modelo el caso real anotado en
//! `docs/objetivos/objetivo-2/modelado-dominio.md`: la observación 395192703
//! de iNaturalist (visón americano en Lucillos, Toledo) y comprueba que no se
//! pierde ningún dato que necesiten HU001 y HU002.

use std::collections::BTreeSet;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use biovigia::dominio::avistamiento::{Avistamiento, EvidenciaFotografica, Fuente, GradoCalidad};
use biovigia::dominio::coordenada::{Coordenada, Latitud, Longitud};
use biovigia::dominio::depredacion::{Depredacion, FaseVital};
use biovigia::dominio::especie::{Especie, EspecieId, NombreCientifico};

const ID_OBSERVACION: &str = "395192703";
const LATITUD: f64 = 40.0400030897;
const LONGITUD: f64 = -4.6470027317;
/// 2026-08-28T10:44:30+02:00 (campo `time_observed_at` de iNaturalist).
const SEGUNDOS_UNIX: u64 = 1_787_906_670;
const FOTOS: [u64; 6] = [
    724599150, 724599181, 724599207, 724599227, 724599243, 724599258,
];

fn url_foto(id: u64) -> String {
    format!("https://inaturalist-open-data.s3.amazonaws.com/photos/{id}/original.jpg")
}

fn vison() -> Especie {
    Especie::new(
        EspecieId::new(1),
        NombreCientifico::new("Neogale vison").unwrap(),
        vec![NombreCientifico::new("Neovison vison").unwrap()],
        "visón americano".to_owned(),
        true,
    )
    .unwrap()
}

fn cangrejo() -> Especie {
    Especie::new(
        EspecieId::new(2),
        NombreCientifico::new("Procambarus clarkii").unwrap(),
        vec![],
        "cangrejo rojo americano".to_owned(),
        true,
    )
    .unwrap()
}

fn avistamiento_del_vison(id_original: &str) -> Avistamiento {
    let evidencia =
        EvidenciaFotografica::new(Some("Marco C".to_owned()), FOTOS.map(url_foto).to_vec())
            .unwrap();
    Avistamiento::new(
        Fuente::INaturalist,
        id_original.to_owned(),
        vison().id(),
        UNIX_EPOCH + Duration::from_secs(SEGUNDOS_UNIX),
        Coordenada::new(
            Latitud::new(LATITUD).unwrap(),
            Longitud::new(LONGITUD).unwrap(),
        ),
        Some(GradoCalidad::Investigacion),
        evidencia,
    )
    .unwrap()
}

#[test]
fn el_avistamiento_conserva_los_campos_que_piden_las_hu() {
    let avistamiento = avistamiento_del_vison(ID_OBSERVACION);

    // Campos de HU001 y HU002: especie, fecha, latitud, longitud, fuente e id.
    assert_eq!(avistamiento.especie(), vison().id());
    assert_eq!(
        avistamiento.fecha(),
        UNIX_EPOCH + Duration::from_secs(SEGUNDOS_UNIX)
    );
    assert!(avistamiento.fecha() < SystemTime::now());
    assert_eq!(avistamiento.coordenada().latitud().grados(), LATITUD);
    assert_eq!(avistamiento.coordenada().longitud().grados(), LONGITUD);
    assert_eq!(avistamiento.fuente(), Fuente::INaturalist);
    assert_eq!(avistamiento.id_original(), ID_OBSERVACION);

    // Datos que permiten fiarse de la identificación.
    assert_eq!(avistamiento.calidad(), Some(GradoCalidad::Investigacion));
    assert_eq!(avistamiento.evidencia().autor(), "Marco C");
    assert_eq!(avistamiento.evidencia().urls().len(), FOTOS.len());
    assert_eq!(avistamiento.evidencia().urls()[0], url_foto(FOTOS[0]));
}

#[test]
fn la_especie_se_reconoce_por_su_nombre_vigente_y_por_el_anterior() {
    let especie = vison();

    assert_eq!(especie.nombre_vigente().como_str(), "Neogale vison");
    assert_eq!(especie.nombres_anteriores()[0].como_str(), "Neovison vison");
    assert_eq!(especie.nombre_comun(), "visón americano");
    assert!(especie.es_invasora());
}

#[test]
fn la_misma_observacion_recibida_dos_veces_es_un_solo_avistamiento() {
    let original = avistamiento_del_vison(ID_OBSERVACION);
    let repetido = avistamiento_del_vison(" 395192703 ");

    assert_eq!(original, repetido);
}

#[test]
fn el_vison_depreda_a_la_especie_objetivo_de_antonio() {
    let fases = BTreeSet::from([FaseVital::Juvenil, FaseVital::Adulto]);
    let depredacion = Depredacion::new(vison().id(), cangrejo().id(), fases.clone(), None).unwrap();

    assert_eq!(depredacion.depredador(), vison().id());
    assert_eq!(depredacion.presa(), cangrejo().id());
    assert_eq!(depredacion.fases_afectadas(), &fases);
}
