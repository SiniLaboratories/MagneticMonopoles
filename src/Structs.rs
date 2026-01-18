use serde::{Serialize, Deserialize};
use std::ops::{Add, Mul};


//Struct of a single physical point in space
#[derive(Default,Serialize, Deserialize, Clone)]
pub struct Point {
    pub rho_em: f64,
    pub v_em: [f64; 3],
    pub rho_ep: f64,
    pub v_ep: [f64; 3],
    pub rho_mm: f64,
    pub v_mm: [f64; 3],
    pub rho_mp: f64,
    pub v_mp: [f64; 3],

    pub E: [f64; 3],
    pub B: [f64; 3]
}

//Struct of the time derivative of a physical point in space
#[derive(Default, Clone)]
pub struct DtPoint {
    pub rho_em: f64,
    pub v_em: [f64; 3],
    pub rho_ep: f64,
    pub v_ep: [f64; 3],
    pub rho_mm: f64,
    pub v_mm: [f64; 3],
    pub rho_mp: f64,
    pub v_mp: [f64; 3],

    pub E: [f64; 3],
    pub B: [f64; 3]
}

// 1. Implementation of DtPoint * f64 -> Point
impl Mul<f64> for DtPoint {
    type Output = Point;

    fn mul(self, dt: f64) -> Self::Output {
        Point {
            rho_em: self.rho_em * dt,
            v_em: [self.v_em[0] * dt,self.v_em[1] * dt,self.v_em[2] * dt],

            rho_ep: self.rho_ep * dt,
            v_ep: [self.v_ep[0] * dt,self.v_ep[1] * dt,self.v_ep[2] * dt],

            rho_mm: self.rho_mm * dt,
            v_mm: [self.v_mm[0] * dt,self.v_mm[1] * dt,self.v_mm[2] * dt],

            rho_mp: self.rho_mp * dt,
            v_mp: [self.v_mp[0] * dt,self.v_mp[1] * dt,self.v_mp[2] * dt],

            E: [self.E[0] * dt,self.E[1] * dt,self.E[2] * dt],

            B: [self.B[0] * dt,self.B[1] * dt,self.B[2] * dt]
        }
    }
}

// 2. Implementation of Point + Point -> Point
impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            rho_em: self.rho_em + rhs.rho_em,
            v_em: [self.v_em[0] + rhs.v_em[0],self.v_em[1] + rhs.v_em[1],self.v_em[2] + rhs.v_em[2]],

            rho_ep: self.rho_ep + rhs.rho_ep,
            v_ep: [self.v_ep[0] + rhs.v_ep[0],self.v_ep[1] + rhs.v_ep[1],self.v_ep[2] + rhs.v_ep[2]],

            rho_mm: self.rho_mm + rhs.rho_mm,
            v_mm: [self.v_mm[0] + rhs.v_mm[0],self.v_mm[1] + rhs.v_mm[1],self.v_mm[2] + rhs.v_mm[2]],

            rho_mp: self.rho_mp + rhs.rho_mp,
            v_mp: [self.v_mp[0] + rhs.v_mp[0],self.v_mp[1] + rhs.v_mp[1],self.v_mp[2] + rhs.v_mp[2]],

            E: [self.E[0] + rhs.E[0],self.E[1] + rhs.E[1],self.E[2] + rhs.E[2]],
            B: [self.B[0] + rhs.B[0],self.B[1] + rhs.B[1],self.B[2] + rhs.B[2]],
        }
    }
}