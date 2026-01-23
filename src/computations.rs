//use std::ops::Add;
//use std::ops::Mul;

pub fn perform_computation_step(struct_array: &Vec<super::structs::Point>, json_config: &serde_json::Value, iteration: u64
) -> Result<Vec<super::structs::Point>, Box<dyn std::error::Error>> 
{
    let nx = json_config["GridParameters"]["nx"].as_u64().unwrap() as usize;
    let ny = json_config["GridParameters"]["ny"].as_u64().unwrap() as usize;
    let nz = json_config["GridParameters"]["nz"].as_u64().unwrap() as usize;

    let ntot = nx*ny*nz;

    let dt = json_config["ComputationParameters"]["dt"].as_f64().unwrap();

    let mut out_array = vec![crate::structs::Point {..Default::default()}; ntot];
    
    let f1 = f(&struct_array,&json_config)?;

    //TODO: parallelize this for loop, it can be run parallelized without problems
    for i in 0..ntot 
    {
        out_array[i] = struct_array[i] + f1[i]*dt;
    }

    //TODO: Implement Runge Kutta 4

    Ok(out_array)
}



pub fn f(struct_array: &Vec<super::structs::Point>, json_config: &serde_json::Value
) -> Result<Vec<super::structs::DtPoint>, Box<dyn std::error::Error>>
{
    let nx = json_config["GridParameters"]["nx"].as_u64().unwrap() as usize;
    let ny = json_config["GridParameters"]["ny"].as_u64().unwrap() as usize;
    let nz = json_config["GridParameters"]["nz"].as_u64().unwrap() as usize;

    let ntot = nx*ny*nz;

    let dx = json_config["GridParameters"]["dx"].as_f64().unwrap();
    let dy = json_config["GridParameters"]["dy"].as_f64().unwrap();
    let dz = json_config["GridParameters"]["dz"].as_f64().unwrap();

    let c = json_config["PhysicalParameters"]["c"].as_f64().unwrap();
    let pi = std::f64::consts::PI;
    let m_em = json_config["PhysicalParameters"]["m_em"].as_f64().unwrap();
    let m_ep = json_config["PhysicalParameters"]["m_ep"].as_f64().unwrap();
    let m_mm = json_config["PhysicalParameters"]["m_mm"].as_f64().unwrap();
    let m_mp = json_config["PhysicalParameters"]["m_mp"].as_f64().unwrap();

    let _relativistic = json_config["ComputationParameters"]["relativistic"].as_bool().unwrap();

    let mut out_array = vec![crate::structs::DtPoint {..Default::default()}; ntot];

    //TODO: parallelize this for loop. It should be able to run flawlessly parallelized
    for i in 0..ntot 
    {
        let (ix,iy,iz) = crate::index_conversion::index_to_3d(i,nx,ny,nz);

        let i_xp1 = crate::index_conversion::index_from_3d(ix+1,iy,iz,nx,ny,nz);
        let i_xp2 = crate::index_conversion::index_from_3d(ix+2,iy,iz,nx,ny,nz);
        let i_xm1 = crate::index_conversion::index_from_3d(ix+nx*2-1,iy,iz,nx,ny,nz);
        let i_xm2 = crate::index_conversion::index_from_3d(ix+nx*2-2,iy,iz,nx,ny,nz);

        let i_yp1 = crate::index_conversion::index_from_3d(ix,iy+1,iz,nx,ny,nz);
        let i_yp2 = crate::index_conversion::index_from_3d(ix,iy+2,iz,nx,ny,nz);
        let i_ym1 = crate::index_conversion::index_from_3d(ix,iy+ny*2-1,iz,nx,ny,nz);
        let i_ym2 = crate::index_conversion::index_from_3d(ix,iy+ny*2-2,iz,nx,ny,nz);

        let i_zp1 = crate::index_conversion::index_from_3d(ix,iy,iz+1,nx,ny,nz);
        let i_zp2 = crate::index_conversion::index_from_3d(ix,iy,iz+2,nx,ny,nz);
        let i_zm1 = crate::index_conversion::index_from_3d(ix,iy,iz+nz*2-1,nx,ny,nz);
        let i_zm2 = crate::index_conversion::index_from_3d(ix,iy,iz+nz*2-2,nx,ny,nz);
        

        //dE/dt --> rotor of B & j_b
        let dBz_dy = (-struct_array[i_yp2].B[2]+8.0*struct_array[i_yp1].B[2]-8.0*struct_array[i_ym1].B[2] + struct_array[i_ym2].B[2])/(12.0*dy);
        let dBy_dz = (-struct_array[i_zp2].B[1]+8.0*struct_array[i_zp1].B[1]-8.0*struct_array[i_zm1].B[1] + struct_array[i_zm2].B[1])/(12.0*dz);
        let dBz_dx = (-struct_array[i_xp2].B[2]+8.0*struct_array[i_xp1].B[2]-8.0*struct_array[i_xm1].B[2] + struct_array[i_xm2].B[2])/(12.0*dx);
        let dBx_dz = (-struct_array[i_zp2].B[0]+8.0*struct_array[i_zp1].B[0]-8.0*struct_array[i_zm1].B[0] + struct_array[i_zm2].B[0])/(12.0*dz);
        let dBy_dx = (-struct_array[i_xp2].B[1]+8.0*struct_array[i_xp1].B[1]-8.0*struct_array[i_xm1].B[1] + struct_array[i_xm2].B[1])/(12.0*dx);
        let dBx_dy = (-struct_array[i_yp2].B[0]+8.0*struct_array[i_yp1].B[0]-8.0*struct_array[i_ym1].B[0] + struct_array[i_ym2].B[0])/(12.0*dy);
        let rotB_x = dBz_dy - dBy_dz;
        let rotB_y = dBz_dx - dBx_dz;
        let rotB_z = dBy_dx - dBx_dy;

        //NOTE: j_e/m_xyz could be implemented inside a struct as function megas, would be easyer. 
        //the total current is determined by two different sus types of flux

        let j_e_x = struct_array[i].j_em()[0] + struct_array[i].j_ep()[0];
        let j_e_y = struct_array[i].j_em()[1] + struct_array[i].j_ep()[1];
        let j_e_z = struct_array[i].j_em()[2] + struct_array[i].j_ep()[2];

        out_array[i].E[0] = c * (rotB_x) - 4.0 * pi * j_e_x; //dEx/dt = +- rotBx/c +- j_m_x/c
        out_array[i].E[1] = c * (rotB_y) - 4.0 * pi * j_e_y; //dEy/dt = +- rotBy/c +- j_m_y/c
        out_array[i].E[2] = c * (rotB_z) - 4.0 * pi * j_e_z; //dEz/dt = +- rotBz/c +- j_m_z/c

        //dB/dt --> rotor of E & j_e
        let dEz_dy = (-struct_array[i_yp2].E[2]+8.0*struct_array[i_yp1].E[2]-8.0*struct_array[i_ym1].E[2] + struct_array[i_ym2].E[2])/(12.0*dy);
        let dEy_dz = (-struct_array[i_zp2].E[1]+8.0*struct_array[i_zp1].E[1]-8.0*struct_array[i_zm1].E[1] + struct_array[i_zm2].E[1])/(12.0*dz);
        let dEz_dx = (-struct_array[i_xp2].E[2]+8.0*struct_array[i_xp1].E[2]-8.0*struct_array[i_xm1].E[2] + struct_array[i_xm2].E[2])/(12.0*dx);
        let dEx_dz = (-struct_array[i_zp2].E[0]+8.0*struct_array[i_zp1].E[0]-8.0*struct_array[i_zm1].E[0] + struct_array[i_zm2].E[0])/(12.0*dz);
        let dEy_dx = (-struct_array[i_xp2].E[1]+8.0*struct_array[i_xp1].E[1]-8.0*struct_array[i_xm1].E[1] + struct_array[i_xm2].E[1])/(12.0*dx);
        let dEx_dy = (-struct_array[i_yp2].E[0]+8.0*struct_array[i_yp1].E[0]-8.0*struct_array[i_ym1].E[0] + struct_array[i_ym2].E[0])/(12.0*dy);
        let rotE_x = dEz_dy - dEy_dz;
        let rotE_y = dEz_dx - dEx_dz;
        let rotE_z = dEy_dx - dEx_dy;

        let j_m_x =  struct_array[i].j_mm()[0] + struct_array[i].j_mp()[0];
        let j_m_y =  struct_array[i].j_mm()[1] + struct_array[i].j_mp()[1];
        let j_m_z =  struct_array[i].j_mm()[2] + struct_array[i].j_mp()[2];

        out_array[i].B[0] = c * (rotE_x) - 4.0 * pi * j_m_x; //dEx/dt = +- rotBx/c +- j_m_x/c
        out_array[i].B[1] = c * (rotE_y) - 4.0 * pi * j_m_y; //dEy/dt = +- rotBy/c +- j_m_y/c
        out_array[i].B[2] = c * (rotE_z) - 4.0 * pi * j_m_z; //dEz/dt = +- rotBz/c +- j_m_z/c
        

        //divegence of rho_em
        let dj_em_x_dx = (-struct_array[i_xp2].j_em()[0]+8.0*struct_array[i_xp1].j_em()[0]-8.0*struct_array[i_xm1].j_em()[0] + struct_array[i_xm2].j_em()[0])/(12.0*dx);
        let dj_em_y_dy = (-struct_array[i_yp2].j_em()[1]+8.0*struct_array[i_yp1].j_em()[1]-8.0*struct_array[i_ym1].j_em()[1] + struct_array[i_ym2].j_em()[1])/(12.0*dy);
        let dj_em_z_dz = (-struct_array[i_zp2].j_em()[2]+8.0*struct_array[i_zp1].j_em()[2]-8.0*struct_array[i_zm1].j_em()[2] + struct_array[i_zm2].j_em()[2])/(12.0*dz);
        let div_j_em = dj_em_x_dx + dj_em_y_dy +dj_em_z_dz;
        out_array[i].rho_em = -div_j_em;

        //divegence of j_ep
        let dj_ep_x_dx = (-struct_array[i_xp2].j_ep()[0]+8.0*struct_array[i_xp1].j_ep()[0]-8.0*struct_array[i_xm1].j_ep()[0] + struct_array[i_xm2].j_ep()[0])/(12.0*dx);
        let dj_ep_y_dy = (-struct_array[i_yp2].j_ep()[0]+8.0*struct_array[i_yp1].j_ep()[1]-8.0*struct_array[i_ym1].j_ep()[1] + struct_array[i_ym2].j_ep()[1])/(12.0*dy);
        let dj_ep_z_dz = (-struct_array[i_zp2].j_ep()[2]+8.0*struct_array[i_zp1].j_ep()[2]-8.0*struct_array[i_zm1].j_ep()[2] + struct_array[i_zm2].j_ep()[2])/(12.0*dz);
        let div_j_ep = dj_ep_x_dx + dj_ep_y_dy +dj_ep_z_dz;
        out_array[i].rho_ep = -div_j_ep;

        //divegence of j_mm
        let dj_mm_x_dx = (-struct_array[i_xp2].j_mm()[0]+8.0*struct_array[i_xp1].j_mm()[0]-8.0*struct_array[i_xm1].j_mm()[0] + struct_array[i_xm2].j_mm()[0])/(12.0*dx);
        let dj_mm_y_dy = (-struct_array[i_yp2].j_mm()[1]+8.0*struct_array[i_yp1].j_mm()[1]-8.0*struct_array[i_ym1].j_mm()[1] + struct_array[i_ym2].j_mm()[1])/(12.0*dy);
        let dj_mm_z_dz = (-struct_array[i_zp2].j_mm()[2]+8.0*struct_array[i_zp1].j_mm()[2]-8.0*struct_array[i_zm1].j_mm()[2] + struct_array[i_zm2].j_mm()[2])/(12.0*dz);
        let div_j_mm = dj_mm_x_dx + dj_mm_y_dy +dj_mm_z_dz;
        out_array[i].rho_mm = -div_j_mm;

        //divegence of j_mp
        let dj_mp_x_dx = (-struct_array[i_xp2].j_mp()[0]+8.0*struct_array[i_xp1].j_mp()[0]-8.0*struct_array[i_xm1].j_mp()[0] + struct_array[i_xm2].j_mp()[0])/(12.0*dx);
        let dj_mp_y_dy = (-struct_array[i_yp2].j_mp()[0]+8.0*struct_array[i_yp1].j_mp()[1]-8.0*struct_array[i_ym1].j_mp()[1] + struct_array[i_ym2].j_mp()[1])/(12.0*dy);
        let dj_mp_z_dz = (-struct_array[i_zp2].j_mp()[2]+8.0*struct_array[i_zp1].j_mp()[2]-8.0*struct_array[i_zm1].j_mp()[2] + struct_array[i_zm2].j_mp()[2])/(12.0*dz);
        let div_j_mp = dj_mp_x_dx + dj_mp_y_dy +dj_mp_z_dz;
        out_array[i].rho_mp = -div_j_mp;

        //lorentz force for em
        let field_for_em = vector_add(struct_array[i].E, cross_product(vector_by_scalar(struct_array[i].v_em, 1.0/c),struct_array[i].B));//vector
        let lorentzian_em = lorentzian(struct_array[i].v_em,c);
        out_array[i].v_em[0] = struct_array[i].rho_em * field_for_em[0] / lorentzian_em / m_em;
        out_array[i].v_em[1] = struct_array[i].rho_em * field_for_em[1] / lorentzian_em / m_em;
        out_array[i].v_em[2] = struct_array[i].rho_em * field_for_em[2] / lorentzian_em / m_em;

        //lorentz force for ep
        let field_for_ep = vector_add(struct_array[i].E, cross_product(vector_by_scalar(struct_array[i].v_ep, 1.0/c),struct_array[i].B));//vector
        let lorentzian_ep = lorentzian(struct_array[i].v_ep,c);
        out_array[i].v_ep[0] = struct_array[i].rho_ep * field_for_ep[0] / lorentzian_ep / m_ep;
        out_array[i].v_ep[1] = struct_array[i].rho_ep * field_for_ep[1] / lorentzian_ep / m_ep;
        out_array[i].v_ep[2] = struct_array[i].rho_ep * field_for_ep[2] / lorentzian_ep / m_ep;

        //lorentz force for mm
        let field_for_mm = vector_add(struct_array[i].B , cross_product(vector_by_scalar(struct_array[i].v_mm, 1.0/c),struct_array[i].E));//vector
        let lorentzian_mm = lorentzian(struct_array[i].v_mm,c);
        out_array[i].v_mm[0] = struct_array[i].rho_mm * field_for_mm[0] / lorentzian_mm / m_mm;
        out_array[i].v_mm[1] = struct_array[i].rho_mm * field_for_mm[1] / lorentzian_mm / m_mm;
        out_array[i].v_mm[2] = struct_array[i].rho_mm * field_for_mm[2] / lorentzian_mm / m_mm;

        //lorentz force for mp
        let field_for_mp = vector_add(struct_array[i].B , cross_product(vector_by_scalar(struct_array[i].v_mp, 1.0/c),struct_array[i].E));//vector
        let lorentzian_mp = lorentzian(struct_array[i].v_mp,c);
        out_array[i].v_mp[0] = struct_array[i].rho_mp * field_for_mp[0] / lorentzian_mp / m_mp;
        out_array[i].v_mp[1] = struct_array[i].rho_mp * field_for_mp[1] / lorentzian_mp / m_mp;
        out_array[i].v_mp[2] = struct_array[i].rho_mp * field_for_mp[2] / lorentzian_mp / m_mp;
    }

    
    Ok(out_array)
}

fn cross_product(a: [f64;3], b: [f64;3]) -> [f64;3]
{
    [
        a[1]*b[2] - a[2]*b[1],
        a[2]*b[0] - a[0]*b[2],
        a[0]*b[1] - a[1]*b[0],
    ]
}

fn norm(v: [f64;3]) -> f64
{
    (v[0]*v[0] + v[1]*v[1] + v[2]*v[2]).sqrt()
}

fn lorentzian(velocity_vector: [f64;3], c: f64) -> f64
{
    let v_norm = norm(velocity_vector);
    if v_norm >= c 
    {
        panic!("Velocity exceeds the speed of light in lorentzian function");
    }
    1.0 / (1.0 - (v_norm*v_norm)/(c*c)).sqrt()  
}

fn vector_add(a: [f64;3], b: [f64;3]) -> [f64;3]
{
    [
        a[0] + b[0],
        a[1] + b[1],
        a[2] + b[2],
    ]
}

fn vector_by_scalar(v: [f64;3], scalar: f64) -> [f64;3]
{
    [
        v[0] * scalar,
        v[1] * scalar,
        v[2] * scalar,
    ]
}