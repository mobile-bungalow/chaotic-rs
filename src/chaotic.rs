// TODO:
// clean up imports (currently my formatter deletes things at random)

use cgmath::{Point3, Vector3};

use glium::*;
use std::collections::VecDeque;

use crate::gui::StringCollection; // contains strings for math evaluation
use std::rc::Rc;

use mexprp::{Context, Expression};

static MAX_BUFLEN: usize = 500usize; // number of points to draw

pub enum ProgramState {
    Start,
    Stopped,
}

#[derive(Copy, Clone)]
pub struct DynVertex {
    pub position: [f64; 3],
    pub color: [f64; 3],
}
implement_vertex!(DynVertex, position, color);


pub struct DynamicSystem {
    pub state: ProgramState,
    pub display: Rc<backend::glutin::Display>,
    pub difeq_ctx: Context<f64>,
    direction: Vector3<f64>,
    vertex_buffer: VertexBuffer<DynVertex>,
    vert_deque: VecDeque<DynVertex>,
    location: Point3<f64>,
    //frequency: f64,
    velocity: f64,
}

impl DynamicSystem {

    pub fn new(window: glium::backend::glutin::Display) -> Self {
        DynamicSystem {
            vertex_buffer: VertexBuffer::empty_dynamic(&window, MAX_BUFLEN)
                .expect("Vertex buffer failed"),
            display: Rc::new(window),
            difeq_ctx: Context::new(),
            direction: Vector3::new(0.0, 0.0, 0.0),
            vert_deque: VecDeque::new(),
            location: Point3::new(1.0, 1.0, 1.0),
            state: ProgramState::Stopped,
            //frequency: 0.0,
            velocity: 0.001,
        }
    }
    /// Computes the indices for the current deque
    /// is an immutable borrow so that it can be called in draw.
    pub fn get_indices(&self) -> glium::IndexBuffer<u16> {

        // builds the stuttering line pattern, 1,2,2,3,3,4 ...
        let mut len = self.vert_deque.len() as u16;
        len = if len > 0 { len } else { 1 };
        let mut indices: Vec<u16> = Vec::new();
        (0..len - 1)
            .into_iter()
            .zip((1..len).into_iter())
            .for_each(|(x, y)| {
                indices.push(x);
                indices.push(y)
            });

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

        // set variable definitions
        // self.difeq_ctx.set_var("x", self.location.x as f64);
        // self.difeq_ctx.set_var("y", self.location.y as f64);
        // self.difeq_ctx.set_var("z", self.location.z as f64);
        // if let mexprp::Answer::Single(x) = Expression::parse_ctx("x + y", self.difeq_ctx.clone()).unwrap().eval().ok().unwrap() {
        //     let x = x as f64;
        // } else {
        //     let x = 0.0;
        // }
        // let expr_y = Expression::parse_ctx("x(0.01 - z) - y", self.difeq_ctx.clone()).unwrap();
        // let expr_z = Expression::parse_ctx("x*y - z", self.difeq_ctx.clone()).unwrap();

        let next_x = self.location.x + self.velocity * 10.0 * (self.location.x + self.location.y);

        let next_y = self.location.y
            + self.velocity * ((self.location.x * (28.0 - self.location.z)) - self.location.y);

        let next_z = self.location.z
            + self.velocity * (self.location.x * self.location.y - self.location.z * 2.666666);

        let current_pos = DynVertex {
            position: [next_x, next_y, next_z],
            color: [1.0, 1.0, 1.0],
        };
        self.location.x = next_x;
        self.location.y = next_y;
        self.location.z = next_z;

        // todo: replace maxbuflen with user defined max shown steps
        match self.state {
            ProgramState::Stopped => return,
            _ => {
                if self.vert_deque.len() >= MAX_BUFLEN {
                    self.vert_deque.pop_back();
                    self.vert_deque.push_front(current_pos);
                } else {
                    self.vert_deque.push_front(current_pos);
                }
            }
        }

    }
    // starts a stopped (default), dynamic system
    pub fn start(&mut self) {
        self.state = ProgramState::Start;
        print!("start\n");
    }

    // freezes updates to the vertex buffer and dynamic system,
    pub fn stop(&mut self) {
        self.state = ProgramState::Stopped;
        print!("stop\n");
    }

    // dumps vertex buffer, resets position to origin, and calls stop
    pub fn reset(&mut self) {
        // do reset things
        self.state = ProgramState::Stopped;
        print!("reset\n");
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
            MAX_BUFLEN,
            DynVertex {
                position: [0.0, 0.0, 0.0],
                color: [1.0, 1.0, 1.0],
            },
        );
        let drain: Vec<DynVertex> = temp.drain(0..MAX_BUFLEN).collect();
        self.vertex_buffer.write(&drain);
    }
    // attempts to resolve the
    // current system as valid or invalid
    pub fn resolve_system(
        &self,
        system: &StringCollection,
    ) -> Result<(), mexprp::errors::ParseError> {

        // let dx: Expression<f64> = Expression::parse(system.dx.to_str())?;
        // let dx: Expression<f64> = Expression::parse(system.dy.to_str())?;
        // let dz: Expression<f64> = Expression::parse(system.dz.to_str())?;


        // TODO: Implement extra functions
        // let fn_vec: &Vec<Expr> = &system
        //     .expressions
        //     .iter()
        //     .map(|(_label, fn_body)| {
        //         let ret_exp: Expr = fn_body.to_str().parse().unwrap();
        //         ret_exp
        //     })
        //     .collect();

        Ok(())

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