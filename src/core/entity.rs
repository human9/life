use glium::{Vertex, VertexBuffer, IndexBuffer};
use glium::index::Index;
use glium::texture::Texture2d;

/// Resources are permanently immutable
struct Resource {
    i: i32, 
}

#[derive(Copy, Clone, Debug)]
pub struct Phat {
    pub position: [f32; 3],
    pub texcoords: [f32; 2],
    pub normal: [f32; 3],
}
implement_vertex!(Phat, position, texcoords, normal);

/// Multiple entites may refer to a resource
struct Entity<'a, V: 'a + Vertex, I: 'a + Index> {
    vo: VertexOption<'a, V, I>,
    to: TextureOption<'a>,
}

enum VertexOption<'a, V: 'a + Vertex, I: 'a + Index> {
    Vertices(&'a VertexBuffer<V>),
    IndexedVertices((&'a VertexBuffer<V>, &'a IndexBuffer<I>)),
    None,
}

enum TextureOption<'a> {
    DiffuseOnly(&'a Texture2d),
    DiffuseAndNormal((&'a Texture2d, &'a Texture2d)),
    None,
}

pub fn foo() {

    let r = Resource {
        i: 2,
    };

}
