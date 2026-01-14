use serde::{Serialize, Deserialize};

//Struct of a single physical point in space
#[derive(Default,Serialize, Deserialize, Clone)]
pub struct Point {
    pub rhoem: f64,
    pub v_em: [f64; 3],
    pub rhoep: f64,
    pub v_ep: [f64; 3],
    pub rhomm: f64,
    pub v_mm: [f64; 3],
    pub rhomp: f64,
    pub v_mp: [f64; 3],

    pub E: [f64; 3],
    pub B: [f64; 3]
}

//Struct of the time derivative of a physical point in space
/*
pub struct DtPoint {
    pub rhoem: f64,
    pub v_em: [f64; 3],
    pub rhoep: f64,
    pub v_ep: [f64; 3],
    pub rhomm: f64,
    pub v_mm: [f64; 3],
    pub rhomp: f64,
    pub v_mp: [f64; 3],

    pub E: [f64; 3],
    pub B: [f64; 3]
}
*/
//Struct of configuration file --> Do I really need it??

