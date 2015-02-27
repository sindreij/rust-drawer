extern crate glium;
extern crate std;
extern crate cgmath;

use std::rc::Rc;
use std::error::Error;

use glium::Surface;

use cgmath::FixedArray;
use cgmath::Matrix;

static identity_matrix: [[f32; 4]; 4] = [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
];

#[uniforms]
#[derive(Copy)]
struct Uniforms {
    color: [f32; 4],
    border_color: [f32; 4],
    matrix: [[f32; 4]; 4],
    center: [f32; 2],
    radius: f32,
}

#[vertex_format]
#[derive(Copy)]
struct Vertex {
    position: [f32; 2],
    coord: [f32; 2],
}

#[derive(Copy, Debug)]
pub struct Rectangle {
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    fill_color: [f32; 4],
    border_color: [f32; 4],
}

#[derive(Copy, Debug)]
pub struct Circle {
    x: f32,
    y: f32,
    radius: f32,
    fill_color: [f32; 4],
    border_color: [f32; 4],
}

#[derive(Copy, Debug)]
pub struct BackgroundGrid {
    pub color: [f32; 4],
}

pub trait Drawable: ToOpenGLLayers {}
impl Drawable for Rectangle {}
impl Drawable for Circle {}
impl Drawable for BackgroundGrid {}

impl Rectangle {
    pub fn new(
            x0: f32,
            y0: f32,
            x1: f32,
            y1: f32,
            fill_color: [f32; 4],
            border_color: [f32; 4]) -> Rectangle {

        Rectangle{
            x0: x0,
            y0: y0,
            x1: x1,
            y1: y1,
            fill_color: fill_color,
            border_color: border_color,
        }
    }
}

impl Circle {
    pub fn new(
            x: f32,
            y: f32,
            radius: f32,
            fill_color: [f32; 4],
            border_color: [f32; 4]) -> Circle {

        Circle{
            x: x,
            y: y,
            radius: radius,
            fill_color: fill_color,
            border_color: border_color,
        }
    }
}

pub struct OpenGLLayer {
    vertex_buffer: Rc<glium::VertexBuffer<Vertex>>,
    index_buffer: Rc<glium::IndexBuffer>,
    matrix: cgmath::Matrix4<f32>,
    program: Rc<glium::Program>,
    uniforms: Uniforms,
}

pub trait ToOpenGLLayers {
    fn to_opengl_layers(&self, display: &glium::Display) -> Vec<OpenGLLayer>;
}

fn get_program(display: &glium::Display) -> glium::Program {
    // GLSL language
    println!("Compiling GLSL");
    glium::Program::from_source(display,
        // vertex shader
        "   #version 330
            uniform mat4 matrix;

            attribute vec2 position;

            void main() {
                gl_Position = vec4(position, 0.0, 1.0) * matrix;
            }
        ",

        // fragment shader
        "   #version 330
            uniform vec4 color;

            void main() {
                gl_FragColor = color;
            }
        ",

        // geometry shader
        None
        ).unwrap()
}

impl OpenGLLayer {
    pub fn draw(&self, target: &mut glium::Frame,
            transformation: & cgmath::Matrix4<f32>) {
        let matrix = self.matrix.mul_m(transformation);
        let uniforms = Uniforms {
            matrix: matrix.into_fixed(),
            .. self.uniforms
        };

        target.draw(
            &*self.vertex_buffer,
            &*self.index_buffer,
            &*self.program,
            &uniforms,
            &glium::DrawParameters {
                blending_function:
                    Some(glium::BlendingFunction::Addition {
                        source: glium::LinearBlendingFactor::SourceAlpha,
                        destination: glium::LinearBlendingFactor::OneMinusSourceAlpha
                    }),
                .. std::default::Default::default()
            });
    }
}

impl ToOpenGLLayers for Rectangle {
    fn to_opengl_layers(
            &self,
            display: &glium::Display,
            ) -> Vec<OpenGLLayer> {
        let Rectangle{x0, y0, x1, y1, fill_color, border_color} = *self;

        let vertex_buffer = Rc::new(glium::VertexBuffer::new(display,
            vec![
                Vertex { position: [x0, y0], coord: [0.0, 0.0] },
                Vertex { position: [x1, y0], coord: [0.0, 0.0] },
                Vertex { position: [x1, y1], coord: [0.0, 0.0] },
                Vertex { position: [x0, y1], coord: [0.0, 0.0] },
            ]));

        let index_buffer = Rc::new(glium::IndexBuffer::new(display,
            glium::index_buffer::TrianglesList(vec![0u16,1,3,1,2,3])));

        let program = Rc::new(get_program(display));

        let mut layers = Vec::new();

        layers.push(OpenGLLayer {
            vertex_buffer: vertex_buffer.clone(),
            index_buffer: index_buffer.clone(),
            matrix: cgmath::Matrix4::identity(),
            program: program.clone(),
            uniforms: Uniforms {
                color: border_color,
                border_color: [0.0, 0.0, 0.0, 0.0],
                matrix: identity_matrix,
                center: [0.0, 0.0],
                radius: 0.0,
            }
        });

        let border_size = 0.01;

        let vertex_buffer = Rc::new(glium::VertexBuffer::new(display,
            vec![
                Vertex {
                    position: [x0 + border_size, y0 + border_size],
                    coord: [0.0, 0.0] },
                Vertex {
                    position: [x1 - border_size, y0 + border_size],
                    coord: [0.0, 0.0] },
                Vertex {
                    position: [x1 - border_size, y1 - border_size],
                    coord: [0.0, 0.0] },
                Vertex {
                    position: [x0 + border_size, y1 - border_size],
                    coord: [0.0, 0.0] },
            ]));

        layers.push(OpenGLLayer {
            vertex_buffer: vertex_buffer.clone(),
            index_buffer: index_buffer.clone(),
            matrix: cgmath::Matrix4::identity(),
            program: program.clone(),
            uniforms: Uniforms {
                color: fill_color,
                border_color: [0.0, 0.0, 0.0, 0.0],
                matrix: identity_matrix,
                center: [0.0, 0.0],
                radius: 0.0,
            }
        });

        layers
    }
}

fn get_program_circle(display: &glium::Display) -> glium::Program {
    // GLSL language
    println!("Compiling GLSL");
    let program = glium::Program::from_source(display,
        // vertex shader
        "   #version 330
            uniform mat4 matrix;
            uniform vec4 color;
            uniform vec4 border_color;

            in vec2 position;
            in vec2 coord;

            out vec2 uv;
            out vec4 forground_color;
            out vec4 background_color;

            void main() {
                gl_Position = vec4(position, 0.0, 1.0) * matrix;

                uv = coord;

                background_color = border_color;
                background_color[3] = 0.0;
            }
        ",

        // fragment shader
        "   #version 330
            uniform vec4 color;
            uniform vec4 border_color;

            in vec2 uv;
            in vec4 background_color;

            out vec4 secColor;

            void main() {
                float dist = distance(uv, vec2(0.0, 0.0));

                float delta = 0.01;

                float alpha = smoothstep(0.99-delta, 0.99, dist);
                vec4 forground_color = mix(color, border_color, alpha);

                alpha = smoothstep(1.0-delta, 1.0, dist);
                secColor = mix(forground_color, background_color, alpha);
            }
        ",

        // geometry shader
        None
        );

    match program {
        Ok(program) => return program,
        Err(err) => {
            println!("Fikk en feil:");
            println!("{}", err.description());
            println!("{}", err);
            panic!();
        }
    }

}

impl ToOpenGLLayers for Circle {
    fn to_opengl_layers(
            &self,
            display: &glium::Display,
            ) -> Vec<OpenGLLayer> {
        let Circle{x, y, radius, fill_color, border_color} = *self;

        let vertex_buffer = Rc::new(glium::VertexBuffer::new(display,
            vec![
                Vertex { position: [x-radius, y+radius],
                    coord: [-1.0, 1.0] },
                Vertex { position: [x+radius, y+radius],
                    coord: [1.0, 1.0] },
                Vertex { position: [x+radius, y-radius],
                    coord: [1.0, -1.0] },
                Vertex { position: [x-radius, y-radius],
                    coord: [-1.0, -1.0] },
            ]));

        let index_buffer = Rc::new(glium::IndexBuffer::new(display,
            glium::index_buffer::TrianglesList(vec![0u16,1,3,1,2,3])));

        let program = Rc::new(get_program_circle(display));

        vec![
            OpenGLLayer {
                vertex_buffer: vertex_buffer,
                index_buffer: index_buffer,
                matrix: cgmath::Matrix4::identity(),
                program: program.clone(),
                uniforms: Uniforms {
                    color: fill_color,
                    border_color: border_color,
                    matrix: identity_matrix,
                    center: [0.0, 0.0],
                    radius: 0.0,
                }
            }
        ]
    }
}

fn get_program_background_grid(display: &glium::Display) -> glium::Program {
    // GLSL language
    println!("Compiling GLSL");
    let program = glium::Program::from_source(display,
        // vertex shader
        "   #version 330
            uniform mat4 matrix;

            in vec2 position;
            in vec2 coord;

            out vec2 uv;

            void main() {
                gl_Position = vec4(position, 0.0, 1.0);

                vec4 uv_full = vec4(coord, 0.0, 1.0);
                uv = uv_full.xy;
            }
        ",

        // fragment shader
        "   #version 330
            uniform vec4 color;
            uniform vec4 border_color;

            in vec2 uv;
            in vec4 background_color;

            out vec4 secColor;

            void main() {
                // vec2 uv_2 = ((uv + 1.0)/2) * 1500;
                // int uv_x = int(uv_2.x);
                // int uv_y = int(uv_2.y);
                // if (mod(uv_x, 30) == 0 || mod(uv_y, 30) == 0) {
                //     secColor = border_color;
                // } else {
                //     secColor = color;
                // }

                vec4 border_color2 = border_color;
                border_color2[3] = 0.15;

                if (mod(gl_FragCoord.x, 30) < 1 ||
                        mod(gl_FragCoord.y, 30) < 1) {
                    secColor = border_color;
                } else if (mod(gl_FragCoord.x, 15) < 1 ||
                        mod(gl_FragCoord.y, 15) < 1) {
                    secColor = border_color2;
                } else {
                    secColor = color;
                }
            }
        ",

        // geometry shader
        None
        );

    match program {
        Ok(program) => return program,
        Err(err) => {
            println!("Fikk en feil:");
            println!("{}", err.description());
            println!("{}", err);
            panic!();
        }
    }

}

impl ToOpenGLLayers for BackgroundGrid {
    fn to_opengl_layers(
            &self,
            display: &glium::Display) -> Vec<OpenGLLayer> {
        let BackgroundGrid{color} = *self;

        let vertex_buffer = Rc::new(glium::VertexBuffer::new(display,
            vec![
                Vertex { position: [-1.0, 1.0],
                    coord: [-1.0, 1.0] },
                Vertex { position: [ 1.0, 1.0],
                    coord: [1.0, 1.0] },
                Vertex { position: [ 1.0,-1.0],
                    coord: [1.0, -1.0] },
                Vertex { position: [-1.0,-1.0],
                    coord: [-1.0, -1.0] },
            ]));

        let index_buffer = Rc::new(glium::IndexBuffer::new(display,
            glium::index_buffer::TrianglesList(vec![0u16,1,3,1,2,3])));

        let program = Rc::new(get_program_background_grid(display));

        vec![
            OpenGLLayer {
                vertex_buffer: vertex_buffer,
                index_buffer: index_buffer,
                matrix: cgmath::Matrix4::identity(),
                program: program.clone(),
                uniforms: Uniforms {
                    color: [0.0, 0.0, 0.0, 0.0],
                    border_color: color,
                    matrix: identity_matrix,
                    center: [0.0, 0.0],
                    radius: 0.0,
                }
            }
        ]
    }
}
