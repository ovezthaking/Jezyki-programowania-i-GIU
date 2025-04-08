use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use::flo_canvas::*;
use flo_draw::*;

trait Fig {
    fn save(&self, w: &mut dyn Write) -> io::Result<()>;
    fn as_string(&self) -> String;
    fn paint(&self);
}

struct Circle {
    radius: f64,
}

struct Square {
    side: f64,
}

struct Rectangle {
    a: f64,
    b: f64,
}

impl Fig for Circle {
    fn save(&self, w: &mut dyn Write) -> io::Result<()> {
        writeln!(w, "Circle {}", self.radius)
    }

    fn as_string(&self) -> String {
        format!("Circle({})", self.radius)
    }
}

impl Fig for Square {
    fn save(&self, w: &mut dyn Write) -> io::Result<()> {
        writeln!(w, "Square {}", self.side)
    }

    fn as_string(&self) -> String {
        format!("Square({})", self.side)
    }
}

impl Fig for Rectangle {
    fn save(&self, w: &mut dyn Write) -> io::Result<()> {
        writeln!(w, "Rectangle {} {}", self.a, self.b)
    }

    fn as_string(&self) -> String {
        format!("Rectangle({}, {})", self.a, self.b)
    }
}

fn save_figures(figures: &[Box<dyn Fig>], path: &str) -> io::Result<()> {
    let mut file = File::create(path)?;
    for fig in figures {
        fig.save(&mut file)?;
    }
    Ok(())
}

fn load_figures(path: &str) -> io::Result<Vec<Box<dyn Fig>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut figures: Vec<Box<dyn Fig>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<_> = line.split_whitespace().collect();

        match parts.as_slice() {
            ["Circle", r] => {
                if let Ok(radius) = r.parse::<f64>() {
                    figures.push(Box::new(Circle { radius }));
                }
            }
            ["Square", s] => {
                if let Ok(side) = s.parse::<f64>() {
                    figures.push(Box::new(Square { side }));
                }
            }
            ["Rectangle", a, b] => {
                if let (Ok(a), Ok(b)) = (a.parse::<f64>(), b.parse::<f64>()) {
                    figures.push(Box::new(Rectangle { a, b }));
                }
            }
            _ => eprintln!("Nieznana figura: {}", line),
        }
    }

    Ok(figures)
}

pub fn main() {
    with_2d_graphics(|| {
        // Create a window
        let canvas      = create_drawing_window("Circle");

        // Draw a circle
        canvas.draw(|gc| {
            // Set up the canvas
            gc.canvas_height(1000.0);
            gc.center_region(0.0, 0.0, 1000.0, 1000.0);

            // Draw a circle
            gc.new_path();
            gc.circle(500.0, 500.0, 250.0);

            gc.fill_color(Color::Rgba(0.3, 0.6, 0.8, 1.0));
            gc.fill();

            gc.line_width(6.0);
            gc.stroke_color(Color::Rgba(0.0, 0.0, 0.0, 1.0));
            gc.stroke();
        });
    });
}