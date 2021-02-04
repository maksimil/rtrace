use std::f64::consts::TAU;
use std::sync::Arc;

use glium::{glutin, Surface};

pub mod shaders;
pub mod space;

use space::{rtx, Line, Vec2};

const FRAME_DELAY: std::time::Duration = std::time::Duration::from_millis(24);

const RAY_GROUP_COUNT: usize = 8;
const RAY_GROUP_SIZE: usize = 4;

const GROUP_MUL: f64 = TAU / (RAY_GROUP_COUNT as f64);
const RAY_MUL: f64 = GROUP_MUL / (RAY_GROUP_SIZE as f64);

const RAY_LEN: f64 = 0.01;

const MAX_DIST: f64 = 2.0;

// vertex declaration
#[derive(Debug, Copy, Clone)]
struct Vertex {
    position: [f64; 2],
}

glium::implement_vertex!(Vertex, position);

fn main() {
    // shapes
    let barriers = Arc::new(vec![
        Line(Vec2 { x: 0.5, y: 0.5 }, Vec2 { x: -0.5, y: 0.5 }),
        Line(Vec2 { x: -0.5, y: 0.5 }, Vec2 { x: -0.5, y: -0.5 }),
        Line(Vec2 { x: 0.0, y: 0.49 }, Vec2 { x: 0.5, y: 0.4 }),
    ]);

    // event loop and display init
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();

    let display =
        glium::Display::new(wb, cb, &event_loop).expect("Failed to create a graphics window");

    let outline_program = glium::Program::from_source(
        &display,
        shaders::outline::VERTEX,
        shaders::outline::FRAGMENT,
        shaders::outline::GEOMETRY,
    )
    .unwrap();

    let ray_program = glium::Program::from_source(
        &display,
        shaders::ray::VERTEX,
        shaders::ray::FRAGMENT,
        shaders::ray::GEOMETRY,
    )
    .unwrap();

    // character
    let pos = Vec2 { x: 0.0, y: 0.0 };

    // running event loop
    event_loop.run(move |event, _, control_flow| {
        let next_frame_time = std::time::Instant::now() + FRAME_DELAY;
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        // event
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        // rtx render
        {
            let pts = (0..RAY_GROUP_COUNT)
                .map(|group| {
                    let group = group as f64;
                    let barriers = barriers.clone();

                    std::thread::spawn(move || {
                        (0..RAY_GROUP_SIZE)
                            .map(|ray| {
                                let ray = ray as f64;

                                let phi1 = GROUP_MUL * group + RAY_MUL * ray;
                                let phi2 = GROUP_MUL * group + RAY_MUL * (ray + 1.0);

                                let phi1cos = phi1.cos();
                                let phi1sin = phi1.sin();
                                let phi2cos = phi2.cos();
                                let phi2sin = phi2.sin();

                                let ray1 = Vec2 {
                                    x: RAY_LEN * phi1cos,
                                    y: RAY_LEN * phi1sin,
                                };

                                let ray2 = Vec2 {
                                    x: RAY_LEN * phi2cos,
                                    y: RAY_LEN * phi2sin,
                                };

                                let ray1len = rtx(Line(pos, pos + ray1), &barriers, MAX_DIST);
                                let ray2len = rtx(Line(pos, pos + ray2), &barriers, MAX_DIST);

                                (
                                    Vertex {
                                        position: [ray1len * phi1cos, ray1len * phi1sin],
                                    },
                                    Vertex {
                                        position: [ray2len * phi2cos, ray2len * phi2sin],
                                    },
                                    Vertex {
                                        position: [0.0, 0.0],
                                    },
                                )
                            })
                            .fold(Vec::new(), |mut acc, (a, b, c)| {
                                acc.push(a);
                                acc.push(b);
                                acc.push(c);
                                acc
                            })
                    })
                })
                .collect::<Vec<_>>();

            let pts = pts
                .into_iter()
                .map(|vec| vec.join().unwrap())
                .flatten()
                .collect::<Vec<_>>();

            let vbuffer = glium::VertexBuffer::new(&display, &pts).unwrap();

            target
                .draw(
                    &vbuffer,
                    &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                    &ray_program,
                    &glium::uniforms::EmptyUniforms,
                    &Default::default(),
                )
                .unwrap();
        }

        // outline render
        {
            let shape = barriers.iter().fold(Vec::new(), |mut acc, line| {
                let line = (line.0 - pos, line.1 - pos);
                acc.push(Vertex {
                    position: [line.0.x, line.0.y],
                });
                acc.push(Vertex {
                    position: [line.1.x, line.1.y],
                });
                acc
            });

            let vbuffer = glium::VertexBuffer::new(&display, &shape).unwrap();

            // drawing outline
            target
                .draw(
                    &vbuffer,
                    &glium::index::NoIndices(glium::index::PrimitiveType::LinesList),
                    &outline_program,
                    &glium::uniforms::EmptyUniforms,
                    &Default::default(),
                )
                .unwrap();
        }

        target.finish().unwrap();
    });
}
