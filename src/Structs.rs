//Struct of a single physical point in space
pub struct Point {
    pub rhoem: f64,
    pub v_em: [f64; 3] = [0.0, 0.0, 0.0];
    pub rhoep: f64,
    pub v_ep: [f64; 3] = [0.0, 0.0, 0.0];
    pub rhomm: f64,
    pub v_mm: [f64; 3] = [0.0, 0.0, 0.0];
    pub rhomp: f64,
    pub v_mp: [f64; 3] = [0.0, 0.0, 0.0];

    pub E: [f64; 3] = [0.0, 0.0, 0.0];
    pub B: [f64; 3] = [0.0, 0.0, 0.0];
}

//Struct of the time derivative of a physical point in space

pub struct DtPoint {
    pub rhoem: f64,
    pub v_em: [f64; 3] = [0.0, 0.0, 0.0];
    pub rhoep: f64,
    pub v_ep: [f64; 3] = [0.0, 0.0, 0.0];
    pub rhomm: f64,
    pub v_mm: [f64; 3] = [0.0, 0.0, 0.0];
    pub rhomp: f64,
    pub v_mp: [f64; 3] = [0.0, 0.0, 0.0];

    pub E: [f64; 3] = [0.0, 0.0, 0.0];
    pub B: [f64; 3] = [0.0, 0.0, 0.0];
}

//Struct of configuration file --> Do I really need it??

