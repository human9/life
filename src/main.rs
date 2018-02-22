extern crate life;
extern crate sifter;
extern crate petgraph;
extern crate rand;
#[macro_use] extern crate glium;
extern crate cgmath;
use std::env;
use rand::distributions::{Range, Sample};
use cgmath::{Matrix4, Vector2, Vector3, Vector4, Zero, InnerSpace};
use life::*;
use sifter::*;
use gl::cgtraits::*;

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

    /*
    for index in graph.node_indices() {
        println!("{}", graph.node_weight(index).unwrap().0.to_string());
        //let mut neighbors: Vec<usize> = graph.neighbors(index).map(|n| { n.index() }).collect();
    }
    */
    
    let mut isdown = false;
    let (mut m_x ,mut m_y) = (0., 0.);

    let mut scale: f32 = 1.0;
    let mut handler = Handler::new();
    
    handler.set_window_mousemove_cb(|x, y| {
        m_x = x;
        m_y = y;
    });
    handler.set_mousescroll_cb(|x, y| {
        //scale += y;
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
   
        
    let mut between_x = Range::new(-100., 100.);
    let mut between_y = Range::new(-100., 100.);
    let mut rng = rand::thread_rng();

    let mut nodes = {
        let data = graph.node_indices().map(|index| {

            let x = between_x.sample(&mut rng);
            let y = between_y.sample(&mut rng);
            graph[index].pos = Vector2::new(x, y);

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
        blend: glium::Blend::alpha_blending(),
        line_width: Some(2.),
        smooth: Some(glium::draw_parameters::Smooth::Nicest),
        polygon_mode: glium::draw_parameters::PolygonMode::Line,
        .. Default::default()
    };

    // force-directed algorithm
    let (W, H) = (800., 600.);
    let area = W * H;
    // random positions to graph are assigned within area
    let k = (area/graph.node_count() as f32).sqrt();
    // in the paper f_a includes an x, I think it's supposed to be z
    let f_a = |x: f32| { (x*x)/k };
    let f_r = |x: f32| { (k*k)/x };

    let epsilon: f32 = 0.01; // minimal distance

    let temp = 0.02 * area.sqrt();

    let mut iteration = 0;
    let iterations = 50;
    core.mainloop(&mut handler, |frame, delta, matrix| {
        // TODO: Split off an input closure in the same style as this.
        // Handlers must be generated on a per-loop basis, otherwise borrowing doesn't work

        // core will hand you the frame, time delta, and a projection matrix
        // you hand the core an input handler

        let mvp = matrix * Matrix4::from_translation(Vector3::new(400., 300., 0.,)) * Matrix4::from_scale(scale);
        let edge_uniforms = uniform! { mvp: mvp.as_uniform(), rgba: Vector4::<f32>::new(0.0 as f32, 0.0, 0.0, 0.7).as_uniform()};
        let node_uniforms = uniform! { mvp: mvp.as_uniform(), rgba: Vector4::<f32>::new(0.0 as f32, 0.6, 0.0, 0.0).as_uniform()};

        if iteration < iterations {
            for v in graph.node_indices() {
                graph[v].disp = Vector2::zero();
                for u in graph.node_indices() {
                    if u != v {
                        let diff = graph[v].pos - graph[u].pos;
                        let magnitude = f32::max(diff.magnitude(), epsilon);
                        graph[v].disp = graph[v].disp + (diff/magnitude) * f_r(magnitude); 
                    }
                }
            }

            for e in graph.edge_indices() {
                let (v, u) = graph.edge_endpoints(e).unwrap();
                let diff = graph[v].pos - graph[u].pos;
                let magnitude = f32::max(diff.magnitude(), epsilon);
                graph[v].disp = graph[v].disp - (diff/magnitude) * f_a(magnitude);
                graph[u].disp = graph[u].disp + (diff/magnitude) * f_a(magnitude);
            }

            for v in graph.node_indices() {
                let magnitude = f32::max(graph[v].disp.magnitude(), epsilon);
                graph[v].pos = graph[v].pos + (graph[v].disp / magnitude) * f32::min(magnitude, temp);
                // uncomment the following to force nodes not to go beyond window space
                //graph[v].pos.x = f32::min(W/2., f32::max(-W/2., graph[v].pos.x));
                //graph[v].pos.y = f32::min(H/2., f32::max(-H/2., graph[v].pos.y));
            }

            let temp = (1.0 - (iteration as f32 / iterations as f32)) * 0.1 * area.sqrt();

            if iteration == iterations - 1 {
                println!("Layout complete!");
            }
        }
        

        {
            let mut mapping = nodes.map();
            // zip with nodelist
            for (node, v) in mapping.iter_mut().zip(graph.node_indices()) {
                let pos = graph[v].pos;

                node.offset[0] = pos.x;
                node.offset[1] = pos.y;
            }
        }

        frame.clear_color(0.1, 0.1, 0.1, 0.0);
        frame.draw((&nodes, &zero), &edges, &program, &edge_uniforms, &lineparams).unwrap();
        frame.draw((&square, nodes.per_instance().unwrap()), &indices, &program, &node_uniforms, &Default::default()).unwrap();

        iteration += 1;
    });

}
