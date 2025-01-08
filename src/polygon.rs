use glium::backend::glutin::SimpleWindowBuilder;
use glium::glutin::surface::WindowSurface;
use glium::index::NoIndices;
use glium::index::PrimitiveType::LineLoop;
use glium::uniforms::{EmptyUniforms, UniformsStorage};
use glium::winit::event::Event::AboutToWait;
use glium::winit::event::{Event, WindowEvent};
use glium::winit::event_loop::EventLoop;
use glium::{implement_vertex, uniform, Display, Frame, Program, Surface, VertexBuffer};
use std::f32::consts::PI;
use std::thread;
use std::time::Duration;
use WindowEvent::Resized;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 4],
}
implement_vertex!(Vertex, position, color);

fn draw(
    display: &Display<WindowSurface>,
    target: &mut Frame,
    program: &Program,
    uniforms: UniformsStorage<[[f32; 4]; 4], EmptyUniforms>,
    sections: f32,
    depth: i32,
) {
    let angle = (2.0 * PI) / sections;

    let mut shape: Vec<Vertex> = Vec::new();
    for section in 0..sections.floor() as i32 {
        shape.push(Vertex {
            position: [
                ((angle * section as f32) + PI/2.0).cos(),
                ((angle * section as f32) + PI/2.0).sin()
            ],
            color: [1.0, 1.0, 1.0, 1.0],
        })
    }

    for _d in 0..depth {
        target
            .draw(
                &VertexBuffer::new(display, &shape).unwrap(),
                &NoIndices(LineLoop),
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();

        let mut shape_t: Vec<Vertex> = Vec::new();
        for section in 0..sections.floor() as usize {
            if section + 1 < sections.floor() as usize {
                shape_t.push(Vertex {
                    position: [
                        (shape[section].position[0] + shape[section + 1].position[0]) / 2f32,
                        (shape[section].position[1] + shape[section + 1].position[1]) / 2f32,
                    ],
                    color: [1.0, 1.0, 1.0, 1.0],
                })
            } else {
                shape_t.push(Vertex {
                    position: [
                        (shape[section].position[0] + shape[0].position[0]) / 2f32,
                        (shape[section].position[1] + shape[0].position[1]) / 2f32,
                    ],
                    color: [1.0, 1.0, 1.0, 1.0],
                })
            }
        }

        shape = shape_t
    }
}

pub(crate) fn nested_polygon(polygon_sides: i32, max_number_of_polygons: i32) {
    let event_loop = EventLoop::builder().build().expect("event loop building");
    let (window, display) = SimpleWindowBuilder::new()
        .with_title("Nested Polygon")
        .with_inner_size(2000, 2000)
        .build(&event_loop);

    let program = Program::from_source(
        &display,
        r#"
        #version 140

        in vec2 position;
        in vec4 color;      // our new attribute
        out vec4 vertex_color;

        uniform mat4 matrix;

        void main() {
            vertex_color = color; // we need to set the value of each `out` variable.
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#,
        r#"
        #version 140

        in vec4 vertex_color;
        out vec4 color;

        void main() {
            color = vec4(vertex_color);
        }
    "#,
        None,
    )
        .unwrap();

    let mut depth = 0;
    let mut delay = 1000;

    #[allow(deprecated)]
    event_loop
        .run(move |ev, window_target| match ev {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::RedrawRequested => {
                    thread::sleep(Duration::from_millis(delay));
                    if delay > 100 {
                        delay -= 20
                    };
                    depth += 1;
                    let mut target = display.draw();
                    target.clear_color(0.005, 0.005, 0.005, 1.0);

                    let uniforms = uniform! {
                        matrix: [
                            [1.0, 0.0, 0.0, 0.0],
                            [0.0, 1.0, 0.0, 0.0],
                            [0.0, 0.0, 1.0, 0.0],
                            [0.0, 0.0, 0.0, 1.0f32],
                        ]
                    };
                    draw(&display, &mut target, &program, uniforms, polygon_sides as f32, depth);
                    target.finish().unwrap();
                    if depth >= max_number_of_polygons {
                        window_target.exit();
                    }
                }
                Resized(window_size) => {
                    display.resize(window_size.into());
                }
                WindowEvent::CloseRequested => {
                    window_target.exit();
                }
                _ => (),
            },
            AboutToWait => {
                window.request_redraw();
            }
            _ => (),
        })
        .unwrap();
}
