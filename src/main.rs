use std::{thread, time::Duration};

use circular_buffer::CircularBuffer;
use dollar_templates::get_templates;
use enigo::*;
use euclid::default::Point2D;
use guessture::{Path2D, PathCoord, NUM_POINTS};
mod dollar_templates;
mod guessture;


fn main() {
    let enigo = Enigo::new(&Settings::default()).unwrap();
    let templates: Vec<guessture::Template> = get_templates();

    // let mut path = Path2D::empty();
    let mut points = CircularBuffer::<100, Point2D<PathCoord>>::new();
    loop {
        let (x, y) = enigo.location().unwrap();
        let b = enigo.button().unwrap();
        points.push_back(Point2D::new(x as f32, y as f32));

        // println!("{:?}", points.to_vec());
        let path = Path2D::new(points.to_vec());
        let res = guessture::find_matching_template_with_defaults(&templates, &path);
        match res {
            Ok(r) => {
                let acc = r.1;
                let name = &r.0.name;
                
                if acc > 0.8 && path.vec_length() > 50 {
                    println!("accuracy: {:?}, pattern: {:?}, l: {:?}", acc, name, &path.vec_length());
                    points.clear();
                }
            }
            Err(_) => {},
        };

        // if path.vec_length() >= NUM_POINTS {
        //     path = Path2D::empty();
        // }

        // thread::sleep(Duration::from_millis(50));
    }
}