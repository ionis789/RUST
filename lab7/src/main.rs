use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

// --- Functia de comparare (mutata aici pentru a fi globala) ---
// Functie helper pentru a compara f64 cu o marja de eroare
fn eq_rel(x: f64, y: f64) -> bool {
    (x - y).abs() < 0.001
}

// --- Definirea Structurii Complex ---

#[derive(Debug, Copy, Clone)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

// --- Implementarea functiilor asociate ---

impl Complex {
    /// Creeaza un numar complex nou din tipuri generice care pot fi convertite in f64
    pub fn new<T, U>(real: T, imag: U) -> Self
    where
        T: Into<f64>,
        U: Into<f64>,
    {
        Complex {
            real: real.into(),
            imag: imag.into(),
        }
    }

    /// Returneaza conjugatul numarului complex
    pub fn conjugate(&self) -> Self {
        Complex {
            real: self.real,
            imag: -self.imag,
        }
    }
}

// --- Implementarea PartialEq pentru assert_eq! ---

impl PartialEq for Complex {
    /// Compara doua numere complexe folosind functia `eq_rel`
    fn eq(&self, other: &Self) -> bool {
        eq_rel(self.real, other.real) && eq_rel(self.imag, other.imag)
    }
}

// --- Implementarea Trait-ului `From` ---

impl From<i32> for Complex {
    fn from(n: i32) -> Self {
        Complex {
            real: n.into(),
            imag: 0.0,
        }
    }
}

impl From<f64> for Complex {
    fn from(n: f64) -> Self {
        Complex {
            real: n,
            imag: 0.0,
        }
    }
}

// --- Implementarea Operatorilor (Generic) ---

// impl Add<T> for Complex
impl<T: Into<Complex>> Add<T> for Complex {
    type Output = Complex;
    fn add(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Complex {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}

// impl Sub<T> for Complex
impl<T: Into<Complex>> Sub<T> for Complex {
    type Output = Complex;
    fn sub(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Complex {
            real: self.real - rhs.real,
            imag: self.imag - rhs.imag,
        }
    }
}

// impl Mul<T> for Complex
impl<T: Into<Complex>> Mul<T> for Complex {
    type Output = Complex;
    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        // (a+bi)(c+di) = (ac-bd) + (ad+bc)i
        let real_part = self.real * rhs.real - self.imag * rhs.imag;
        let imag_part = self.real * rhs.imag + self.imag * rhs.real;
        Complex {
            real: real_part,
            imag: imag_part,
        }
    }
}

// impl Neg for Complex
impl Neg for Complex {
    type Output = Complex;
    fn neg(self) -> Self::Output {
        Complex {
            real: -self.real,
            imag: -self.imag,
        }
    }
}

// --- Implementarea `Display` ---

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let real = self.real;
        let imag = self.imag;
        
        let real_is_zero = eq_rel(real, 0.0);
        let imag_is_zero = eq_rel(imag, 0.0);

        match (real_is_zero, imag_is_zero) {
            // Caz: 0+0i -> "0"
            (true, true) => write!(f, "0"),
            
            // Caz: 7+0i -> "7"
            (false, true) => write!(f, "{}", real),
            
            // Caz: 0+5i -> "5i"
            (true, false) => write!(f, "{}i", imag),
            
            // Caz: 1+2i sau 1-2i
            (false, false) => {
                if imag < 0.0 {
                    // "1-2i" (folosim abs() pentru a evita "1+-2i")
                    write!(f, "{}-{}i", real, imag.abs())
                } else {
                    // "1+2i"
                    write!(f, "{}+{}i", real, imag)
                }
            }
        }
    }
}


// --- BONUS: Operatori Assign ---

impl<T: Into<Complex>> AddAssign<T> for Complex {
    fn add_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.real += rhs.real;
        self.imag += rhs.imag;
    }
}

impl<T: Into<Complex>> SubAssign<T> for Complex {
    fn sub_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.real -= rhs.real;
        self.imag -= rhs.imag;
    }
}

impl<T: Into<Complex>> MulAssign<T> for Complex {
    fn mul_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        let old_real = self.real;
        let old_imag = self.imag;
        
        self.real = old_real * rhs.real - old_imag * rhs.imag;
        self.imag = old_real * rhs.imag + old_imag * rhs.real;
    }
}


// --- CODUL DE TESTARE (Nemodificat) ---

// This is a macro that panics if 2 floats are not equal using an epsilon.
// You are not required to understand it yet, just to use it.
macro_rules! assert_eq_rel {
    ($x:expr, $y: expr) => {
        let x = $x as f64;
        let y = $y as f64;
        let r = eq_rel(x, y);
        assert!(r, "{} != {}", x, y);
    };
}

fn main() {
    let a = Complex::new(1.0, 2.0);
    assert_eq_rel!(a.real, 1);
    assert_eq_rel!(a.imag, 2);

    let b = Complex::new(2.0, 3);
    let c = a + b;
    assert_eq_rel!(c.real, 3);
    assert_eq_rel!(c.imag, 5);

    let d = c - a;
    assert_eq!(b, d);

    let e = (a * d).conjugate();
    assert_eq_rel!(e.imag, -7);

    let f = (a + b - d) * c;
    assert_eq!(f, Complex::new(-7, 11));

    // Note: .to_string() uses Display to format the type
    assert_eq!(Complex::new(1, 2).to_string(), "1+2i");
    assert_eq!(Complex::new(1, -2).to_string(), "1-2i");
    assert_eq!(Complex::new(0, 5).to_string(), "5i");
    assert_eq!(Complex::new(7, 0).to_string(), "7");
    assert_eq!(Complex::new(0, 0).to_string(), "0");

    let h = Complex::new(-4, -5);
    let i = h - (h + 5) * 2.0;
    assert_eq_rel!(i.real, -6);

    let j = -i + i;
    assert_eq_rel!(j.real, 0);
    assert_eq_rel!(j.imag, 0);

    println!("ok!");
}