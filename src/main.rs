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
use glium::Surface;
use life::core::Core;
use life::gl::cgtraits::AsUniform;

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

    let mut scale: f32 = 1.0;
    let mut translation = Vector3::new(0., 0., 0.,);
    let mut movement = Vector3::new(0., 0., 0.,);

    let zero = glium::VertexBuffer::new(&core.window.clone_display(), &vec![gl::base::Vertex3D{position:[0.,0.,0.,]}]).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
   
        
    let mut between_x = Range::new(-100., 100.);
    let mut between_y = Range::new(-100., 100.);
    let mut rng = rand::thread_rng();
    
    let mut square;
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
        let mut colouredindices = Vec::new();
        graph.edge_indices().for_each(|index| {
            let ends = graph.edge_endpoints(index).unwrap();
            if graph[index] == "OVERLAY" {
                colouredindices.push(ends.0.index() as u32);
                colouredindices.push(ends.1.index() as u32);
            } else {
                edgeindices.push(ends.0.index() as u32);
                edgeindices.push(ends.1.index() as u32);
            }
        });

        (glium::IndexBuffer::new(&core.window.clone_display(), glium::index::PrimitiveType::LinesList, &edgeindices).unwrap(),
        glium::IndexBuffer::new(&core.window.clone_display(), glium::index::PrimitiveType::LinesList, &colouredindices).unwrap())
    };


    let lineparams = glium::DrawParameters {
        blend: glium::Blend::alpha_blending(),
        //line_width: Some(2.),
        //smooth: Some(glium::draw_parameters::Smooth::Nicest),
        polygon_mode: glium::draw_parameters::PolygonMode::Line,
        .. Default::default()
    };


    // force-directed algorithm
    let (mut w, mut h) = (800., 600.);
    let area = w * h;
    // random positions to graph are assigned within area
    let k = (area/graph.node_count() as f32).sqrt();
    // in the paper f_a includes an x, I think it's supposed to be z
    let f_a = |x: f32| { (x*x)/k };
    let f_r = |x: f32| { (k*k)/x };

    let epsilon: f32 = 0.01; // minimal distance

    let temp = 0.02 * area.sqrt();

    let mut iteration = 0;
    let iterations = 50;

    let mut shutdown = false;
    let display = core.window.clone_display();

    let mut projection: Matrix4<f32> = Matrix4::from(cgmath::Ortho {
        left: -w/2.,
        right: w/2.,
        bottom: -h/2.,
        top: h/2.,
        near: -1.0,
        far: 1.0 });

    let mut mousedown = false;
    let (mut m_x, mut m_y) = (0.0, 0.0);
    loop {

        {
            use glium::glutin::{DeviceEvent, WindowEvent, Event, ElementState, MouseButton, MouseScrollDelta};

            core.window.events_loop.poll_events(|e| {
                match e {
                    Event::DeviceEvent { event, .. } => {
                        match event {

                            DeviceEvent::MouseWheel { delta } => {
                                let (mut _x, mut y);
                                match delta {
                                    MouseScrollDelta::LineDelta(lx, ly) => {_x = lx; y = ly},
                                    MouseScrollDelta::PixelDelta(lx, ly) => {_x = lx; y = ly},
                                }
                                scale += y / 100.;
                                if scale < 1. {
                                    scale = 1.;
                                }
                            },

                            _ => (),
                        }
                    },

                    Event::WindowEvent { event, .. } => match event {

                        WindowEvent::CursorMoved { position, .. } => {
                            if mousedown {
                                movement.x += m_x - position.0 as f32;
                                movement.y += m_y - position.1 as f32;
                            }
                            // this will always be set, and up to date
                            m_x = position.0 as f32;
                            m_y = position.1 as f32;
                        },

                        WindowEvent::Resized(x, y) => {

                            mousedown = false;

                            w = x as f32;
                            h = y as f32;

                            projection = Matrix4::from(cgmath::Ortho {
                                left: -w/2.,
                                right: w/2.,
                                bottom: -h/2.,
                                top: h/2.,
                                near: -1.0,
                                far: 1.0 });
                        },

                        WindowEvent::Closed => {
                            shutdown = true;
                        },

                        WindowEvent::MouseInput { button, state, .. } => {
                            match button {
                                MouseButton::Left => {
                                    match state {
                                        ElementState::Pressed => {
                                            if m_x > 0.0 && m_y > 0.0 {
                                                mousedown = true;
                                            }
                                        },
                                        ElementState::Released => {
                                            mousedown = false;
                                        },
                                    }
                                },
                                _ => {},
                            }
                        },

                        _ => (),

                    },

                    _ => (),

                }
            });

        }

        translation.x -= movement.x;
        translation.y += movement.y;
        movement.x = 0.;
        movement.y = 0.;

        let mvp = projection * Matrix4::from_translation(translation) * Matrix4::from_scale(scale.powf(scale));
        let edge_uniforms = uniform! { mvp: mvp.as_uniform(), rgba: Vector4::<f32>::new(0.0 as f32, 0.0, 0.0, 0.7).as_uniform()};
        let edge_uniforms2 = uniform! { mvp: mvp.as_uniform(), rgba: Vector4::<f32>::new(1.0 as f32, 0.0, 0.0, 0.7).as_uniform()};
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
                // this didn't seem to be faster at all...
                /*graph[v].disp = graph.raw_nodes().into_par_iter()
                    .fold(|| Vector2::zero(), |mut vector, u| {
                        let diff = graph[v].pos - u.weight.pos;
                        let magnitude = f32::max(diff.magnitude(), epsilon);
                        vector = vector + (diff/magnitude) * f_r(magnitude);
                        vector
                    }).sum();*/
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

        square = core.window.with_display(gl::base::make_square).expect("Failed making a triangle!");
        {
            let mut mapping = square.map();
            // zip with nodelist
            for vertex in mapping.iter_mut() {
                let scalesq = scale.powf(scale);
                vertex.position[0] /= scalesq;
                vertex.position[1] /= scalesq;
                vertex.position[2] /= scalesq;
            }
        }

        let mut frame = display.draw();

        frame.clear_color(0.1, 0.1, 0.1, 0.0);
        frame.draw((&nodes, &zero), &edges.0, &program, &edge_uniforms, &lineparams).unwrap();
        frame.draw((&nodes, &zero), &edges.1, &program, &edge_uniforms2, &lineparams).unwrap();
        frame.draw((&square, nodes.per_instance().unwrap()), &indices, &program, &node_uniforms, &Default::default()).unwrap();

        iteration += 1;

        frame.finish().unwrap();

        if shutdown {
            break;
        }
    }

}
