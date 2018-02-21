extern crate life;
extern crate sifter;
extern crate petgraph;
extern crate rand;
#[macro_use] extern crate glium;
extern crate cgmath;
use std::env;
use rand::distributions::{Range, Sample};
use cgmath::Matrix4;
use life::*;
use sifter::*;

use petgraph::Graph;
use petgraph::graph::NodeIndex;
use glium::glutin::{ElementState, MouseButton};
use glium::Surface;
use life::core::Core;
use life::gl::cgtraits::AsUniform;
use life::core::window::Handler;

fn main() {
    let mut core = Core::initialize();

    // let life core handle the mainloop

    let mut args = env::args();
    args.next(); // consume first useless arg
    let filename = match args.next() {
        Some(arg) => arg,
        None => panic!("Failed to open file"),
    };
    
    let siffile = read_file(&filename).unwrap();
    let mut mapped_graph = sif_to_petgraph(&siffile);
    let ref mut graph = mapped_graph.graph;

    for index in graph.node_indices() {
        println!("{}", graph.node_weight(index).unwrap().0.to_string());
        //let mut neighbors: Vec<usize> = graph.neighbors(index).map(|n| { n.index() }).collect();
    }
    
    let mut isdown = false;
    let (mut m_x ,mut m_y) = (0., 0.);

    let mut handler = Handler::new();
    
    handler.set_window_mousemove_cb(|x, y| {
        m_x = x;
        m_y = y;
    });
    handler.set_mouseclick_cb(|button, state| {
        if button == MouseButton::Left {
            match state {
                Pressed => isdown = true,
                Released => isdown = false,
            }
        }
    });


    let square = core.window.with_display(gl::base::make_square).expect("Failed making a triangle!");
    let zero = glium::VertexBuffer::new(&core.window.clone_display(), &vec![gl::base::Vertex3D{position:[0.,0.,0.,]}]).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
   
        
    let mut between = Range::new(100., 600.);
    let mut rng = rand::thread_rng();

    let mut nodes = {
        let data = graph.node_indices().map(|index| {

            let x = between.sample(&mut rng);
            let y = between.sample(&mut rng);
            graph[index].1 = x;
            graph[index].2 = y;

            gl::base::Offset {
                offset: [x, y, 0.0],
            }
        }).collect::<Vec<_>>();
        glium::vertex::VertexBuffer::dynamic(&core.window.clone_display(), &data).unwrap()
    };

    let program = core.window.with_display(gl::base::compile_debug_program).unwrap();

    let edges = { 
        let mut edgeindices = Vec::new();
        graph.edge_indices().for_each(|index| {
            let ends = graph.edge_endpoints(index).unwrap();
            edgeindices.push(ends.0.index() as u32);
            edgeindices.push(ends.1.index() as u32);
        });

        glium::IndexBuffer::new(&core.window.clone_display(), glium::index::PrimitiveType::LinesList, &edgeindices).unwrap()
    };


    let lineparams = glium::DrawParameters {
        line_width: Some(1.),
        //smooth: Some(glium::draw_parameters::Smooth::Nicest),
        polygon_mode: glium::draw_parameters::PolygonMode::Line,
        .. Default::default()
    };

    core.mainloop(&mut handler, |frame, delta, matrix| {

        let uniforms = uniform! { mvp: matrix.as_uniform() };

        // todo - implement force directed algo
        // make graph contain pos, disp, etc
        for v in graph.node_indices() {
            for u in graph.node_indices() {
                
            }
        }

        {
            let mut mapping = nodes.map();
            // zip with nodelist
            for node in mapping.iter_mut() {
                //node.offset[0] += 1.0;
            }
        }

        frame.clear_color(0.1, 0.1, 0.1, 0.0);
        frame.draw((&square, nodes.per_instance().unwrap()), &indices, &program, &uniforms, &Default::default()).unwrap();
        frame.draw((&nodes, &zero), &edges, &program, &uniforms, &lineparams).unwrap();
    });

}
