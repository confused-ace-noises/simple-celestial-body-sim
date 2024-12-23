use std::{fmt::Display, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign}};

use bevy::math::FloatPow;

#[derive(Debug, Clone, Copy)]
pub struct SciFloat {
    mantissa: f32,
    exp: i32,
}
impl SciFloat {
    pub fn new(value: f32) -> SciFloat {
        if value == 0.0 {
            return SciFloat { mantissa: 0.0, exp: 0 };
        }

        let exp = value.log(10.0).floor() as i32;
        let mantissa = value / 10f32.powi(exp);
        SciFloat { mantissa, exp: exp }
    }

    pub fn add_exp(&mut self, exp: i32) {
        self.exp += exp;
    }

    fn incr(&mut self) {
        self.mantissa *= 10_f32;
        self.exp -= 1;
    }

    pub fn normalize(&mut self) {
        if self.mantissa == 0.0 {
            self.exp = 0;
            return;
        }

        let log = self.mantissa.log10().floor() as i32;
        
        self.mantissa /= 10_f32.powi(log);
        self.exp += log;
    }
}

impl Neg for SciFloat {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            mantissa: -self.mantissa,
            exp: self.exp
        }
    }
}

impl Add for SciFloat {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let exp_diff = self.exp - rhs.exp;

        match exp_diff {
            0 => {
                let mant = self.mantissa + rhs.mantissa;
                let exp = mant.log(10.0).floor() as i32;
                let mantissa = mant / 10_f32.powi(exp);

                Self {
                    mantissa,
                    exp: self.exp + exp
                }
            }

            x if x > 0 => {
                let mut one = self;

                for _ in 0..x {
                    one.incr();
                };

                let mant = one.mantissa + rhs.mantissa;
                let exp = mant.log(10.0).floor() as i32;
                let mantissa = mant / 10_f32.powi(exp);

                Self {
                    mantissa,
                    exp: one.exp + exp
                }
            },
            
            x if x < 0 => {
                let mut two = rhs;

                for _ in 0..(-x) {
                    two.incr();
                };
                
                let mant = two.mantissa + self.mantissa;
                let exp = mant.log(10.0).floor() as i32;
                let mantissa = mant / 10_f32.powi(exp);

                Self {
                    mantissa,
                    exp: two.exp + exp
                }
            },

            _ => panic!("this shouldn't happen..."),
        }
    }
}

impl AddAssign for SciFloat {
    fn add_assign(&mut self, rhs: Self) {
        let x = self.clone() + rhs;

        self.mantissa = x.mantissa;
        self.exp =  x.exp;
    }
}

impl Sub for SciFloat {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl SubAssign for SciFloat {
    fn sub_assign(&mut self, rhs: Self) {
        let x = self.clone() - rhs;

        self.mantissa = x.mantissa;
        self.exp =  x.exp;
    }
}

impl Mul for SciFloat {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut x = Self {
            mantissa: self.mantissa * rhs.mantissa,
            exp: self.exp + rhs.exp
        };

        x.normalize();
        x
    }
}

impl MulAssign for SciFloat {
    fn mul_assign(&mut self, rhs: Self) {
        self.mantissa = self.mantissa * rhs.mantissa;
        self.exp = self.exp + rhs.exp;
    }
}

impl Div for SciFloat {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let mut x = Self {
            mantissa: self.mantissa / rhs.mantissa,
            exp: self.exp - rhs.exp
        };

        x.normalize();
        x
    }
}

impl DivAssign for SciFloat {
    fn div_assign(&mut self, rhs: Self) {
        self.mantissa = self.mantissa / rhs.mantissa;
        self.exp = self.exp - rhs.exp;
    }
}

impl PartialEq for SciFloat {
    fn eq(&self, other: &Self) -> bool {
        self.mantissa == other.mantissa && self.exp == other.exp
    }
}

impl Display for SciFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}e{}", self.mantissa, self.exp)
    }
}

impl From<f32> for SciFloat {
    fn from(value: f32) -> Self {
        Self::new(value)
    }
}

impl From<SciFloat> for f32 {
    fn from(value: SciFloat) -> Self {
        value.mantissa.powi(value.exp)
    }
}

impl FloatPow for SciFloat {
    fn squared(self) -> Self {
        Self {
            mantissa: self.mantissa,
            exp: self.exp*2
        }
    }

    fn cubed(self) -> Self {
        Self {
            mantissa: self.mantissa,
            exp: self.exp*3
        }
    }
}

impl SciFloat {
    pub fn powi(self, exp: i32) -> Self {
        Self {
            mantissa: self.mantissa,
            exp: self.exp*exp
        }
    }

    pub fn sqrti(self, index: i32) -> Self {
        Self {
            mantissa: self.mantissa,
            exp: self.exp/index
        }
    }
}

#[test]
fn test_sci_float() {
    let scifloat1 = SciFloat::new(7000000000000.0);
    let mut scifloat2 = SciFloat::new(100000000.0);


    println!("{:?} \n {:?}", scifloat1, scifloat2);

    scifloat2 += scifloat1;

    // let x = scifloat2 + scifloat1;
    //let x = scifloat1 + scifloat2;
    println!("{:?}", scifloat2)
}