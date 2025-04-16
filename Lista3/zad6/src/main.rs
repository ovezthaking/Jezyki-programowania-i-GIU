use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use::flo_canvas::*;
use flo_draw::*;
use rand::Rng;



trait Fig: Send + Sync {
    fn save(&self, w: &mut dyn Write) -> io::Result<()>;
    fn as_string(&self) -> String;
    fn paint(&self, canvas: &DrawingTarget);
}

struct Circle {
    radius: f64,
}

struct Square {
    a: f64,
}

struct Rectangle {
    a: f64,
    b: f64,
}


fn random_color() -> Color{
    let mut rng = rand::rng();
    let r = rng.random_range(0.0..=1.0); 
    let g = rng.random_range(0.0..=1.0); 
    let b = rng.random_range(0.0..=1.0); 
    let a = 1.0; 
    Color::Rgba(r, g, b, a)
}

impl Fig for Circle {
    fn save(&self, w: &mut dyn Write) -> io::Result<()> {
        writeln!(w, "Circle {}", self.radius)
    }

    fn as_string(&self) -> String {
        format!("Circle({})", self.radius)
    }

    fn paint(&self, canvas: &DrawingTarget) {
        let radius = self.radius;
        canvas.draw(|gc| {
            gc.clear_canvas(Color::Rgba(1.0, 1.0, 1.0, 1.0)); 

            gc.canvas_height(1000.0);
            gc.center_region(0.0, 0.0, 1000.0, 1000.0);

            gc.new_path();
            gc.circle(500.0, 500.0, radius as f32);

            gc.fill_color(random_color());
            gc.fill();

            gc.line_width(6.0);
            gc.stroke_color(Color::Rgba(0.0, 0.0, 0.0, 1.0));
            gc.stroke();
        });
    }

}

impl Fig for Square {
    fn save(&self, w: &mut dyn Write) -> io::Result<()> {
        writeln!(w, "Square {}", self.a)
    }

    fn as_string(&self) -> String {
        format!("Square({})", self.a)
    }

    fn paint(&self, canvas: &DrawingTarget) {
        let a = self.a;
        canvas.draw(|gc| {
            gc.clear_canvas(Color::Rgba(1.0, 1.0, 1.0, 1.0)); 

            gc.canvas_height(1000.0);
            gc.center_region(0.0, 0.0, 1000.0, 1000.0);

            gc.new_path();
            gc.rect(500.0, 500.0, a as f32 + 500.0, a as f32 + 500.0);

            gc.fill_color(random_color());
            gc.fill();

            gc.line_width(6.0);
            gc.stroke_color(Color::Rgba(0.0, 0.0, 0.0, 1.0));
            gc.stroke();
        });
    }
}

impl Fig for Rectangle {
    fn save(&self, w: &mut dyn Write) -> io::Result<()> {
        writeln!(w, "Rectangle {} {}", self.a, self.b)
    }

    fn as_string(&self) -> String {
        format!("Rectangle({}, {})", self.a, self.b)
    }

    fn paint(&self, canvas: &DrawingTarget) {
        let a = self.a;
        let b = self.b;
        canvas.draw(|gc| {
            gc.clear_canvas(Color::Rgba(1.0, 1.0, 1.0, 1.0)); // czyszczenie

            gc.canvas_height(1000.0);
            gc.center_region(0.0, 0.0, 1000.0, 1000.0);

            gc.new_path();
            gc.rect(5.0, 5.0, a as f32 + 5.0, b as f32 + 5.0);

            gc.fill_color(random_color());
            gc.fill();

            gc.line_width(6.0);
            gc.stroke_color(Color::Rgba(0.0, 0.0, 0.0, 1.0));
            gc.stroke();
        });
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
                if let Ok(a) = s.parse::<f64>() {
                    figures.push(Box::new(Square { a }));
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





fn main() {
    let circle = Circle { radius: 50.0 };
    let square = Square { a: 100.0 };
    let rectangle = Rectangle { a: 150.0, b: 300.0 };
    let rectangle2 = Rectangle { a: 800.0, b: 600.0 };
    let circle2 = Circle {radius: 300.0};
    let figures: Vec<Box<dyn Fig>> = vec![Box::new(circle), Box::new(square), Box::new(rectangle), Box::new(rectangle2), Box::new(circle2)];
    save_figures(&figures, "figury.txt").unwrap();

    let loaded_figures = load_figures("figury.txt").unwrap();

    with_2d_graphics(move || {

        let canvas = create_drawing_window("Figury");
        for fig in &loaded_figures {
        println!("{}", fig.as_string());
        fig.paint(&canvas);
        println!("Naciśnij Enter, żeby wyświetlić następną figurę...");
        let _ = io::stdin().read_line(&mut String::new());
            
            
        }
        std::process::exit(0); 
    });
}