pub struct Color
{
    red: f64,
    green: f64,
    blue: f64
}

impl Color
{
    pub fn new(r: f64, g: f64, b: f64) -> Color
    {
        assert!(r >=  0.0 || r <= 1);
        assert!(g >=  0.0 || g <= 1);
        assert!(b >=  0.0 || b <= 1);

        return Color {red: r, green: g, blue: b};
    }
}
