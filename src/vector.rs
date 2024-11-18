use std::cell::OnceCell;

use color_eyre::eyre::{eyre, Result};

use crate::lazy::Lazy;

pub struct FromToVector3D {
    from: Vector3D,
}

impl FromToVector3D {
    pub fn to(&self, destination: &Vector3D) -> Vector3D {
        destination.subtract(&self.from)
    }
}

#[derive(Debug, Clone)]
pub struct Vector3D {
    x: f64,
    y: f64,
    z: f64,
    len: Lazy<f64>,
    squid: Lazy<f64>,
}

impl std::fmt::Display for Vector3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {}, y: {}, z: {})", self.x(), self.y(), self.z())
    }
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3D {
            x,
            y,
            z,
            len: Lazy::Lazy(OnceCell::new()),
            squid: Lazy::Lazy(OnceCell::new()),
        }
    }

    pub fn try_new(x: f64, y: f64, z: f64) -> Result<Self> {
        let (x, y, z) = (x, y, z);

        if x.abs() > f64::MAX || y.abs() > f64::MAX || z.abs() > f64::MAX {
            Err(eyre!(format!(
                "Maximum allowed input dimension for Vector3D is {}",
                f64::MAX
            )))
        } else {
            Ok(Vector3D {
                x,
                y,
                z,
                len: Lazy::Lazy(OnceCell::new()),
                squid: Lazy::Lazy(OnceCell::new()),
            })
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }

    fn length(&self) -> f64 {
        self.len
            .get_or_init(self.len.get_or_init(self.squid().sqrt()))
    }

    // "Squid" is a funny name for "Squared Euclidean distance"
    pub fn squid(&self) -> f64 {
        self.squid
            .get_or_init((self.x.abs()).powi(2) + (self.y.abs()).powi(2) + (self.z.abs()).powi(2))
    }

    pub fn dot(&self, operand: &Vector3D) -> f64 {
        (self.x * operand.x) + (self.y * operand.y) + self.z * operand.z
    }

    pub fn cross(&self, operand: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.y * operand.z - self.z * operand.y,
            y: operand.z * self.x - self.x * operand.z,
            z: self.x * operand.y - self.y * operand.x,
            len: Lazy::Lazy(OnceCell::new()),
            squid: Lazy::Lazy(OnceCell::new()),
        }
    }

    pub fn divide(&self, divisor: f64) -> Vector3D {
        if divisor == 0. {
            return Vector3D {
                x: 0.,
                y: 0.,
                z: 0.,
                len: Lazy::Lazy(OnceCell::new()),
                squid: Lazy::Lazy(OnceCell::new()),
            };
        }

        Vector3D {
            x: self.x / divisor,
            y: self.y / divisor,
            z: self.z / divisor,
            len: Lazy::Lazy(OnceCell::new()),
            squid: Lazy::Lazy(OnceCell::new()),
        }
    }

    pub fn unit(&self) -> Vector3D {
        self.divide(self.length())
    }

    pub fn invert(&self) -> Vector3D {
        Vector3D {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            len: Lazy::Lazy(OnceCell::new()),
            squid: Lazy::Lazy(OnceCell::new()),
        }
    }

    pub fn add(&self, addend: &Vector3D) -> Self {
        Vector3D {
            x: self.x + addend.x,
            y: self.y + addend.y,
            z: self.z + addend.z,
            len: Lazy::Lazy(OnceCell::new()),
            squid: Lazy::Lazy(OnceCell::new()),
        }
    }

    pub fn subtract(&self, subtrahend: &Vector3D) -> Self {
        Vector3D {
            x: self.x - subtrahend.x,
            y: self.y - subtrahend.y,
            z: self.z - subtrahend.z,
            len: Lazy::Lazy(OnceCell::new()),
            squid: Lazy::Lazy(OnceCell::new()),
        }
    }

    pub fn scale(&self, factor: f64) -> Self {
        Vector3D {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
            len: Lazy::Lazy(OnceCell::new()),
            squid: Lazy::Lazy(OnceCell::new()),
        }
    }

    pub fn to(&self, destination: &Vector3D) -> Self {
        self.subtract(destination)
    }

    pub fn from(origin: &Vector3D) -> FromToVector3D {
        FromToVector3D {
            from: origin.into(),
        }
    }
}

impl From<&Vector3D> for Vector3D {
    fn from(value: &Vector3D) -> Self {
        Vector3D {
            x: value.x,
            y: value.y,
            z: value.z,
            len: Lazy::Lazy(OnceCell::new()),
            squid: Lazy::Lazy(OnceCell::new()),
        }
    }
}

pub const X: Vector3D = Vector3D {
    x: 1.,
    y: 0.,
    z: 0.,
    len: Lazy::Eager(1.),
    squid: Lazy::Eager(1.),
};

pub const Y: Vector3D = Vector3D {
    x: 0.,
    y: 1.,
    z: 0.,
    len: Lazy::Eager(1.),
    squid: Lazy::Eager(1.),
};

pub const Z: Vector3D = Vector3D {
    x: 0.,
    y: 0.,
    z: 1.,
    len: Lazy::Eager(1.),
    squid: Lazy::Eager(1.),
};

pub const O: Vector3D = Vector3D {
    x: 0.,
    y: 0.,
    z: 0.,
    len: Lazy::Eager(0.),
    squid: Lazy::Eager(0.),
};
