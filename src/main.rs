use std::f32::consts::TAU;
use std::sync::{Arc, RwLock};
use std::time::Instant;

use glium::{glutin, Surface};

pub mod shaders;
pub mod space;
pub mod vertex;

use vertex::{RayVertex, Vertex};

use space::{rtx, Line, Vec2};

const FRAME_DELAY: std::time::Duration = std::time::Duration::from_millis(24);

const RAY_GROUP_COUNT: usize = 16;
const RAY_GROUP_SIZE: usize = 4;

const RAY_COUNT: usize = RAY_GROUP_COUNT * RAY_GROUP_SIZE;

const DP: f32 = TAU / (RAY_COUNT as f32);

const RAY_LEN: f32 = 0.01;

const MAX_DIST: f32 = 2.0;

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

    // ray data buffer
    let ray_data = Arc::new(RwLock::new(
        (0..RAY_COUNT)
            .map(|i| {
                let p = DP * (i as f32);
                RayVertex {
                    ray: [p.cos(), p.sin()],
                    rad: 1.0,
                }
            })
            .collect::<Vec<_>>(),
    ));

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
            let start = Instant::now();
            let threads = (0..RAY_GROUP_COUNT)
                .map(|group| {
                    let barriers = barriers.clone();
                    let ray_data = ray_data.clone();

                    std::thread::spawn(move || {
                        (0..RAY_GROUP_SIZE)
                            .map(|ray| {
                                let idx = group * RAY_GROUP_SIZE + ray;
                                let ray = {
                                    let ray_data = ray_data.read().unwrap();

                                    Vec2 {
                                        x: RAY_LEN * ray_data[idx].ray[0],
                                        y: RAY_LEN * ray_data[idx].ray[1],
                                    }
                                };

                                let raylen = rtx(Line(pos, pos + ray), &barriers, MAX_DIST);
                                (idx, raylen)
                            })
                            .collect::<Vec<_>>()
                    })
                })
                .collect::<Vec<_>>()
                .into_iter()
                .map(|thread| thread.join().unwrap())
                .collect::<Vec<_>>();

            let mut ray_data = ray_data.write().unwrap();
            for thread in threads {
                for (idx, rad) in thread {
                    ray_data[idx].rad = rad;
                }
            }

            println!("Elapsed: {}ms", start.elapsed().as_millis());

            let vbuffer = glium::VertexBuffer::new(&display, &ray_data).unwrap();

            target
                .draw(
                    &vbuffer,
                    &glium::index::NoIndices(glium::index::PrimitiveType::LineStrip),
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
