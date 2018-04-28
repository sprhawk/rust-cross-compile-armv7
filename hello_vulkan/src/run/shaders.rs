
#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
}
impl_vertex!(Vertex, position);


pub mod default_vertex_shader {
    #[derive(VulkanoShader)]
    #[ty = "vertex"]
    #[path = "src/run/default_vertex.glsl"]
    #[allow(dead_code)]
    struct Dummy;
}

pub mod default_fragment_shader {
    #[derive(VulkanoShader)]
    #[ty = "fragment"]
    #[path = "src/run/default_fragment.glsl"]
    #[allow(dead_code)]
    struct Dummy;
}