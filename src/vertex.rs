use glium::implement_vertex;

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 2],
}

implement_vertex!(Vertex, position);

#[derive(Debug, Clone, Copy)]
pub struct RayVertex {
    pub ray: [f32; 2],
    pub rad: f32,
}

implement_vertex!(RayVertex, ray, rad);
