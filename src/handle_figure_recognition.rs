use crate::guessture;
use circular_buffer::CircularBuffer;
use enigo::*;
use crate::figures_templates::get_templates;
use euclid::default::Point2D;
use guessture::{Path2D, PathCoord};
use std::{thread::{self, JoinHandle}, time::Duration};

const CIRCULAR_BUFFER_LEN: usize = 100;
const POLLING_RATE_MS: u64 = 10;


/**
   Recognizes figures drawn with the mouse thanks to the $1 algorithm. 
   The list of figures is the one defines in figures_template.rs.
   N.B. The drawing order of the figures must be the same as shown here 
   "https://depts.washington.edu/acelab/proj/dollar/index.html"
   
   `on_figure_recognized(figure_name)` - callback called when a figure is recognized
 */
pub fn recognize_figures(mut on_figure_recognized: impl FnMut(&String) -> () + 'static + Send) -> JoinHandle<()> {
    thread::spawn(move || {
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
    })
}