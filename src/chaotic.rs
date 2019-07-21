use cgmath::{Point3, Vector3};

use glium::*;
use std::collections::VecDeque;

use std::rc::Rc;
//use meval::{Context, Expr};
static MAX_BUFLEN: usize = 255usize;

#[derive(Copy, Clone)]
pub struct DynVertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}
implement_vertex!(DynVertex, position, color);


pub struct DynamicSystem {
    pub vertex_buffer: VertexBuffer<DynVertex>,
    pub direction: Vector3<f32>,
    pub display: Rc<backend::glutin::Display>,
    vert_deque: VecDeque<DynVertex>,
    location: Point3<f32>,
    //frequency: f32,
    velocity: f32,
}

impl DynamicSystem {

    pub fn new(window: glium::backend::glutin::Display) -> Self {
        DynamicSystem {
            vertex_buffer: VertexBuffer::empty_dynamic(&window, MAX_BUFLEN)
                .expect("Vertex buffer failed"),
            display: Rc::new(window),
            direction: Vector3::new(0.0, 0.0, 0.0),
            vert_deque: VecDeque::new(),
            location: Point3::new(0.0, 0.0, 0.0),
            //frequency: 0.0,
            velocity: 0.0,
        }
    }
    /// Computes the indices for the current deque
    /// is an immutable borrow so that it can be called in draw.
    pub fn get_indices(&self) -> glium::IndexBuffer<u16> {

        // builds the stuttering line pattern, 1,2,2,3,3,4 ...
        let mut len = self.vert_deque.len() as u16;
        len = if len > 0 { len } else { 1 };
        let indices: Vec<u16> = (0..len - 1)
            .into_iter()
            .chain((1..len).into_iter())
            .collect();

        let index_buffer = glium::IndexBuffer::new(
            &(*self.display),
            glium::index::PrimitiveType::LinesList,
            &indices,
        )
        .unwrap();

        index_buffer
    }

    pub fn get_vertices(&self) -> glium::vertex::VertexBufferSlice<DynVertex> {
        self.vertex_buffer.slice(0..self.vert_deque.len()).unwrap()
    }
    // pushes an updated point on to the dynamic system
    fn update_system(&mut self) {
        // todo: replace maxbuflen with user defined max shown steps
        if self.vert_deque.len() >= MAX_BUFLEN {
            let old = self.vert_deque.pop_front().unwrap().position;
            self.vert_deque.push_back(DynVertex {
                position: [0.01 + old[0], 0.01 + old[1], 0.0],
                color: [1.0, 1.0, 1.0],
            });
        } else {
            self.location += self.direction * self.velocity;
            self.vert_deque.push_back(DynVertex {
                position: [self.location.x, self.location.y, self.location.z],
                color: [1.0, 1.0, 1.0],
            });
        }

    }


    // updates the vertex buffer to accurately
    // reflect an updated dynamic system
    pub fn update_vertex_buffer(&mut self) {

        self.update_system(); //update the coordinates of the system
                              //something like this to update all the color
                              // must be cloned or else it will pass it a mutible buffer
                              // that is constantly being resized. and it borks.
        let mut temp = self.vert_deque.clone();
        //todo: Make this copy less memory
        temp.resize(
            255,
            DynVertex {
                position: [0.0, 0.0, 0.0],
                color: [1.0, 1.0, 1.0],
            },
        );
        let drain: Vec<DynVertex> = temp.drain(0..255).collect();
        self.vertex_buffer.write(&drain);
    }

}


#[cfg(test)]
mod test {
    //    use super::*;

    // fn initialize_dummy() -> DynamicSystem {
    //     unimplemented!();
    // }

    #[test]
    fn update_buffer_valid() {
        unimplemented!();
    }

    #[test]
    fn update_system_valid() {
        unimplemented!();
    }

}