use crate::guessture;
use circular_buffer::CircularBuffer;
use enigo::*;
use crate::figures_templates::get_templates;
use euclid::default::Point2D;
use guessture::{Path2D, PathCoord};
use std::{thread, time::Duration};

const CIRCULAR_BUFFER_LEN: usize = 100;
const POLLING_RATE_MS: u64 = 10;


pub fn recognize_figures(on_figure_recognized: fn(figure_name: &String) -> ()) {
    let enigo: Enigo = Enigo::new(&Settings::default()).unwrap();
    let templates: Vec<guessture::Template> = get_templates();

    let mut points = CircularBuffer::<CIRCULAR_BUFFER_LEN, Point2D<PathCoord>>::new();
    loop {
        let (x, y) = enigo.location().unwrap();
        points.push_back(Point2D::new(x as f32, y as f32));

        let path = Path2D::new(points.to_vec());
        let res = guessture::find_matching_template_with_defaults(&templates, &path);
        match res {
            Ok(r) => {
                let acc = r.1;
                let name = &r.0.name;
                
                if acc > 0.9 {
                    on_figure_recognized(name);
                    points.clear();
                }
            },
            Err(_) => {},
        };

        thread::sleep(Duration::from_millis(POLLING_RATE_MS));
    }
}