use color_eyre::eyre::{eyre, Result};
use std::cell::OnceCell;

#[derive(Debug, Clone)]
enum Lazy<T> {
    Lazy(OnceCell<T>),
    Eager(T),
}

impl<T: Copy> Lazy<T> {
    fn get_or_init(&self, value: T) -> T {
        match &self {
            Lazy::Lazy(inner) => *inner.get_or_init(|| value),
            Lazy::Eager(inner) => *inner,
        }
    }
}

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
    x: i32,
    y: i32,
    z: i32,
    len: Lazy<f64>,
    squid: Lazy<u64>,
}

impl std::fmt::Display for Vector3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {}, y: {}, z: {})", self.x(), self.y(), self.z())
    }
}

impl Vector3D {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Vector3D {
            x,
            y,
            z,
            len: Lazy::Lazy(OnceCell::new()),
            squid: Lazy::Lazy(OnceCell::new()),
        }
    }

    pub fn try_new(x: f32, y: f32, z: f32) -> Result<Self> {
        let (x, y, z) = ((x * 1000.) as i64, (y * 1000.) as i64, (z * 1000.) as i64);

        if x.abs() > i32::MAX as i64 || y.abs() > i32::MAX as i64 || z.abs() > i32::MAX as i64 {
            Err(eyre!(format!(
                "Maximum allowed input dimension for Vector3D is {}",
                (i32::MAX / 1000) as f32
            )))
        } else {
            Ok(Vector3D {
                x: x as i32,
                y: y as i32,
                z: z as i32,
                len: Lazy::Lazy(OnceCell::new()),
                squid: Lazy::Lazy(OnceCell::new()),
            })
        }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }
    pub fn z(&self) -> i32 {
        self.z
    }

    fn x_float(&self) -> f32 {
        self.x as f32 / 1000.
    }

    fn y_float(&self) -> f32 {
        self.y as f32 / 1000.
    }
    fn z_float(&self) -> f32 {
        self.z as f32 / 1000.
    }

    fn length(&self) -> f64 {
        self.len
            .get_or_init(self.len.get_or_init((self.squid() as f64).sqrt()))
    }

    fn squid(&self) -> u64 {
        self.squid.get_or_init(
            (self.x.unsigned_abs() as u64).pow(2) / 1000
                + (self.y.unsigned_abs() as u64).pow(2) / 1000
                + (self.z.unsigned_abs() as u64).pow(2) / 1000,
        )
    }

    pub fn dot(&self, operand: &Vector3D) -> i32 {
        ((self.x * operand.x) + (self.y * operand.y) + (self.z * operand.z)) / 1000
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

    pub fn divide(&self, divisor: f32) -> Vector3D {
        let divisor = (divisor * 1000.) as i32;

        if divisor == 0 {
            return Vector3D {
                x: 0,
                y: 0,
                z: 0,
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
        self.divide(self.length() as f32)
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

    pub fn scale(&self, factor: f32) -> Self {
        let factor = (factor * 1000.) as i32;

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
    x: 1i32,
    y: 0i32,
    z: 0i32,
    len: Lazy::Eager(1f64),
    squid: Lazy::Eager(1u64),
};

pub const Y: Vector3D = Vector3D {
    x: 0i32,
    y: 1i32,
    z: 0i32,
    len: Lazy::Eager(1f64),
    squid: Lazy::Eager(1u64),
};

pub const Z: Vector3D = Vector3D {
    x: 0i32,
    y: 0i32,
    z: 1i32,
    len: Lazy::Eager(1f64),
    squid: Lazy::Eager(1u64),
};

pub const O: Vector3D = Vector3D {
    x: 0i32,
    y: 0i32,
    z: 0i32,
    len: Lazy::Eager(0f64),
    squid: Lazy::Eager(0u64),
};
