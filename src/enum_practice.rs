enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn as_str(&self) -> &str {
        match self {
            Color::Red => "Red",
            Color::Green => "Green",
            Color::Blue => "Blue",
        }
    }
}
fn sf() {
    let color = Color::Red;
    let color2 = Color::Blue;
    let color3 = Color::Green;

    let colors = [color, color2, color3];

    for color in colors {
        println!("The color is: {}", color.as_str());
    }
}
