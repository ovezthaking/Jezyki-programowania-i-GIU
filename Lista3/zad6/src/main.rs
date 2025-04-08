use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use flo_draw::*;
use flo_canvas::*;
use std::time::Duration;

trait Fig {
    fn save(&self, w: &mut dyn Write) -> io::Result<()>;
    fn as_string(&self) -> String;
    fn paint(&self, canvas: &mut dyn DrawingTarget);
}

struct Circle {
    radius: f64,
    x: f64,
    y: f64,
}

struct Square {
    side: f64,
    x: f64,
    y: f64,
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


fn main() -> io::Result<()> {
    let figures: Vec<Box<dyn Fig>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Square { side: 3.2 }),
        Box::new(Circle { radius: 1.5 }),
        Box::new(Rectangle { a:2.0, b: 4.0}),
    ];

    save_figures(&figures, "figury.txt")?;

    let loaded = load_figures("figury.txt")?;
    for fig in loaded {
        println!("{}", fig.as_string());
    }

    Ok(())
}
