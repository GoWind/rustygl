use crate::render_gl::to_radians;
use rayon::prelude::*;
use std::ops;

//Source: http://www.songho.ca/opengl/gl_sphere.html
//Returns an interleaved array with vertex and normal co-ordinates 
//for each vertex and a vertex array with the index of co-ordinates

pub fn generate_sphere(sectors: u32, stacks: u32, radius: f32, normalized: bool) -> (Vec<f32>, Vec<u32>) {
    //phi is the stack angle
    //theta is the sector angle
    let radius_inv: f32 = 1.0 / radius ;
    let sector_step: f32 =  2.0 * 3.14 / (sectors as f32);
    let stack_step : f32 =  3.14/ (stacks as f32);
    
    let mut vertices: Vec<f32> = Vec::new();
    // let mut normals: Vec<f32> = Vec::new();
    // a stack needs 2 sets of points. Ergo
    // n stacks need n+1 sets of points
    // similarly for sectors
    (0.. stacks+1).into_par_iter()
        .for_each(|stack| {
            let stack_angle = (3.14/2.0) - ((stack as f32) * stack_step);
            let xz = radius * stack_angle.cos();
            let y  = radius * stack_angle.sin();
            for j in 0..(sectors+1) {
                let sector_angle = (j as f32) * sector_step;
                let z = xz *  sector_angle.cos();
                let x  = xz * sector_angle.sin();
                if normalized {
                    vertices.push(x * radius_inv);
                    vertices.push(y * radius_inv);
                    vertices.push(z * radius_inv);
                } else {
                    vertices.push(x);
                    vertices.push(y);
                    vertices.push(z);
                }
                let normal_x = x * radius_inv;
                let normal_y = y * radius_inv;
                let normal_z = z * radius_inv;
                vertices.push(normal_x);
                vertices.push(normal_y);
                vertices.push(normal_z);
            }
        });
    let indices = generate_indices(sectors, stacks);
    (vertices, indices)
}

pub fn generate_indices(sectors: u32, stacks: u32) -> Vec<u32> {
    let mut vertex_indices: Vec<u32> = Vec::new();
    let mut triangles = 0;
    for i in 0..stacks {
        let mut k1 = i * (sectors + 1) ;
        let mut k2 = k1 + sectors + 1;
        for j in 0..sectors {
           if i != 0 {
             vertex_indices.push(k1);
             vertex_indices.push(k2);
             vertex_indices.push(k1+1);
             triangles += 1;
           }
           if i != (stacks-1) {
            vertex_indices.push(k1+1);
            vertex_indices.push(k2);
            vertex_indices.push(k2+1);
            triangles += 1;
           }
           k1 += 1;
           k2 += 1;
        }
    }
    println!("num triangles pushed is {}", triangles);
    vertex_indices
}



