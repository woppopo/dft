#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    pub const fn new(real: f64, imag: f64) -> Self {
        Complex { real, imag }
    }

    pub const fn re(&self) -> f64 {
        self.real
    }

    pub const fn im(&self) -> f64 {
        self.imag
    }

    pub fn conjugate(&self) -> Self {
        Self {
            real: self.real,
            imag: -self.imag,
        }
    }

    pub fn abs(&self) -> f64 {
        (self.real.powi(2) + self.imag.powi(2)).sqrt()
    }

    pub fn arg(&self) -> f64 {
        use std::f64::consts::PI;

        match (self.real, self.imag) {
            (x, y) if x > 0.0 => (y / x).atan(),
            (x, y) if x < 0.0 && y >= 0.0 => (y / x).atan() + PI,
            (x, y) if x < 0.0 && y < 0.0 => (y / x).atan() - PI,
            (x, y) if x == 0.0 && y > 0.0 => PI / 2.0,
            (x, y) if x == 0.0 && y < 0.0 => -PI / 2.0,
            (_, _) => f64::NAN,
        }
    }

    pub fn exp(&self) -> Self {
        let exp_real = self.real.exp();
        Self {
            real: self.imag.cos() * exp_real,
            imag: self.imag.sin() * exp_real,
        }
    }
}

impl std::ops::Add for Complex {
    type Output = Self;

    fn add(self, other: Complex) -> Complex {
        Self {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl std::ops::AddAssign for Complex {
    fn add_assign(&mut self, other: Complex) {
        *self = *self + other;
    }
}

impl std::ops::Sub for Complex {
    type Output = Self;

    fn sub(self, other: Complex) -> Complex {
        Self {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }
}

impl std::ops::SubAssign for Complex {
    fn sub_assign(&mut self, other: Complex) {
        *self = *self - other;
    }
}

impl std::ops::Mul for Complex {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        }
    }
}

impl std::ops::MulAssign for Complex {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl std::ops::Div for Complex {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        let Self { real: a, imag: b } = self;
        let Self { real: c, imag: d } = other;

        let denom = c.powi(2) + d.powi(2);
        Self {
            real: (a * c + b * d) / denom,
            imag: (b * c - a * d) / denom,
        }
    }
}

impl std::ops::DivAssign for Complex {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

impl std::ops::Neg for Complex {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            real: -self.real,
            imag: -self.imag,
        }
    }
}

impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({} + {}i)", self.real, self.imag))
    }
}

impl From<f64> for Complex {
    fn from(x: f64) -> Self {
        Self { real: x, imag: 0.0 }
    }
}
