extern crate glium;

use models;
use models::ToOpenGLLayers;

struct DrawingState {
    start_pos: (f32, f32),
    end_pos: (f32, f32),
}


pub struct DrawerLogic<'a> {
    elements: Vec<Box<models::Drawable>>,
    layers: Vec<models::OpenGLLayer>,
    display: &'a glium::Display,
    current_drawing: Option<DrawingState>,
    current_mouse_pos: (f32, f32),
}

impl<'a> DrawerLogic<'a> {
    pub fn new(display: &glium::Display) -> DrawerLogic {
        DrawerLogic{
            elements: Vec::new(), 
            layers: Vec::new(),
            display: display,
            current_drawing: None,
            current_mouse_pos: (0.0, 0.0),
        }
    }

    pub fn get_layers(&self) -> &Vec<models::OpenGLLayer> {
        return &self.layers;
    }

    fn update_layers(&mut self) {
        self.layers.clear();
        for element in self.elements.iter() {
            let new_layers = element.to_opengl_layers(self.display);
            for layer in new_layers.into_iter() {
                self.layers.push(layer);
            }
        }

        if let Some(ref state) = self.current_drawing {
            let (start_x, start_y) = state.start_pos;
            let (end_x, end_y) = state.end_pos;
            let (start_x, end_x) = {
                if (end_x < start_x) {
                    (end_x, start_x)
                } else {
                    (start_x, end_x)
                }
            };
            let (start_y, end_y) = {
                if (end_y < start_y) {
                    (end_y, start_y)
                } else {
                    (start_y, end_y)
                }
            };
            let tmp_element = models::Rectangle::new(
                start_x,
                start_y,
                end_x,
                end_y,
                [0.9, 0.9, 0.9, 1.0],
                [0.0, 0.0, 0.0, 1.0],
            );
            let new_layers = tmp_element.to_opengl_layers(self.display);
            for layer in new_layers.into_iter() {
                self.layers.push(layer);
            }
        }
    }

    pub fn on_mouse_down(&mut self) {
        self.current_drawing = Some(DrawingState{
            start_pos: self.current_mouse_pos,
            end_pos: self.current_mouse_pos,
        })
    }

    pub fn on_mouse_move(&mut self, pos: (f32, f32)) {
        let mut drawing = false;
        if let Some(ref mut state) = self.current_drawing {
            drawing = true;
            state.end_pos = pos;
        }

        if drawing {
            self.update_layers();
        }

        println!("Pos: {:?}", pos);

        self.current_mouse_pos = pos;

    }

    pub fn on_mouse_up(&mut self) {
        if let Some(ref state) = self.current_drawing {
            let (start_x, start_y) = state.start_pos;
            let (end_x, end_y) = state.end_pos;
            let (start_x, end_x) = {
                if (end_x < start_x) {
                    (end_x, start_x)
                } else {
                    (start_x, end_x)
                }
            };
            let (start_y, end_y) = {
                if (end_y < start_y) {
                    (end_y, start_y)
                } else {
                    (start_y, end_y)
                }
            };
            let new_element = models::Rectangle::new(
                start_x,
                start_y,
                end_x,
                end_y,
                [0.9, 0.9, 0.9, 1.0],
                [0.0, 0.0, 0.0, 1.0],
            );
            self.elements.push(Box::new(new_element));
        }

        self.current_drawing = None;

        self.update_layers();
    }
}