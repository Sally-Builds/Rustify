
enum Color {
    RED,
    BLUE,
    YELLOW,
    WHITE,
    BLACK
}

impl Color {
    fn print(&self) {
        match self {
            Color::BLACK => println!("Black"),
            Color::RED => println!("Red"),
            Color::BLUE => println!("Blue"),
            Color::YELLOW => println!("Yellow"),
            Color::WHITE => println!("White"),
        };
    }
}

struct Dimension {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimension {
    fn print (&self) {
        println!("width {:?}, height {:?}, depth {:?}", self.width, self.height, self.depth);
    }
}

struct ShoppingBox {
    dimension: Dimension,
    weight: f64,
    color: Color,
}

impl ShoppingBox {
    fn new(dimension: Dimension, weight: f64, color: Color) -> Self {
        Self {dimension, weight, color}
    }

    fn print(&self) {
        self.print_color();
        self.print_dimension();
        self.print_weight();
    }

    fn  print_dimension (&self) {
        self.dimension.print();
    }

    fn print_weight (&self) {
        println!("{:?}", self.weight);
    }

    fn print_color (&self) {
       self.color.print();
    }
}

fn main () {
    let dimension = Dimension {
        width: 54.89,
        height: 123.98,
        depth: 12.5
    };

    let shopping_box = ShoppingBox::new(dimension, 23.4, Color::RED);

    // ShoppingBox::print_color(&shopping_box)
    shopping_box.print_color();
    shopping_box.print_dimension();
    shopping_box.print_weight();

    println!("==============================");

    shopping_box.print();
}