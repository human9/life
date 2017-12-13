use cgmath::Matrix4;
use cgmath::Vector3;
use cgmath::BaseNum;

pub trait AsUniform {
    type Uniform;
    fn as_uniform(&self) -> Self::Uniform;
}

impl<T: BaseNum> AsUniform for Matrix4<T> {
    type Uniform = [[T; 4]; 4];
    fn as_uniform(&self) -> Self::Uniform {
        (*self).into()
    }
}

impl<T: BaseNum> AsUniform for Vector3<T> {
    type Uniform = [T; 3];
    fn as_uniform(&self) -> Self::Uniform {
        (*self).into()
    }
}
