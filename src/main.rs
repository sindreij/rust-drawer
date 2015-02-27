#![feature(plugin)]

#![plugin(glium_macros)]
extern crate glium_macros;

extern crate glutin;
extern crate glium;
extern crate cgmath;

use std::old_io::timer;
use std::time::Duration;
use std::f32::consts::PI;
use std::num::Float;

use glium::Surface;
use glium::DisplayBuild;

use models::ToOpenGLLayers;

mod models;

fn main() {
    let display = glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_title(format!("Glium TEST"))
        //.with_multisampling(16)
        .with_vsync()
        .build_glium().unwrap();

    let mut elements: Vec<Box<models::Drawable>> = Vec::new();

    // elements.push(Box::new(models::Rectangle::new(
    //     -1.0,
    //     -1.0,
    //     1.0,
    //     1.0,
    //     [1.0, 0.0, 0.0, 1.0],
    //     [1.0, 1.0, 1.0, 1.0],
    //     )));

    let rectangle = models::Circle::new(
        0.0,
        0.0,
        0.3,
        [0.9, 0.9, 0.9, 1.0],
        [0.0, 0.0, 0.0, 1.0],
        );

    elements.push(Box::new(rectangle));

    println!("Rectangle: {:?}", rectangle);

    let mut layers: Vec<models::OpenGLLayer> = Vec::new();

    for element in elements.iter() {
        let new_layers = element.to_opengl_layers(&display);
        for layer in new_layers.into_iter() {
            layers.push(layer);
        }
    }

    let background = models::BackgroundGrid {
        color: [0.0, 0.0, 0.0, 0.25],
    };

    let background_layers = background.to_opengl_layers(&display);

    'main: loop {
        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => break 'main,
                _ => (),
            }
        }

        let mut target = display.draw();

        let (width, height) = target.get_dimensions();

        let mut ratio_height:f32 = width as f32 / height as f32;
        let mut ratio_width:f32 = height as f32 / width as f32;

        if ratio_width <= 1.0 {
            ratio_height = 1.0;
        } else {
            ratio_width = 1.0;
        }

        let matrix:cgmath::Matrix4<f32> = cgmath::Matrix4::new(
            ratio_width, 0.0, 0.0, 0.0,
            0.0, ratio_height, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );

        target.clear_color(1.0, 1.0, 1.0, 1.0);

        for layer in background_layers.iter() {
            layer.draw(&mut target, &matrix);
        }

        for layer in layers.iter() {
            layer.draw(&mut target, &matrix);
        }


        timer::sleep(Duration::milliseconds(34));

        //rot += PI * 0.001;
    }

}
