
//Function that converts index i to ix iy iz
pub fn index_to_3d(i:usize, nx:usize, ny:usize, nz:usize) -> (usize, usize, usize)
{
    let iz = i / (nx*ny);
    let iy = (i - iz*nx*ny) / nx;
    let ix = i - iz*nx*ny - iy*nx;

    (ix, iy, iz)
}


//Function that converts ix iy iz to index i
pub fn index_from_3d(ix:usize, iy:usize, iz:usize, nx:usize, ny:usize, nz:usize) -> usize
{
    let i = iz*nx*ny + iy*nx + ix;

    i
}