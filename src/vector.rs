use std::cell::OnceCell;

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

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn length(&self) -> f64 {
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
            y: self.z * operand.x - self.x * operand.z,
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

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::utils::approx_eq;

    use super::*;

    use test_case::test_case;

    #[test_case(1.0, 2.5, PI               ; "positive components")]
    #[test_case(-1.0, -2.5, -PI           ; "negative components")]
    #[test_case(1.0, -2.5, PI              ; "mixed components")]
    #[test_case(0.0, 0.0, 0.0              ; "zero components")]
    fn test_new_and_components(x: f64, y: f64, z: f64) {
        let v = Vector3D::new(x, y, z);
        assert!(approx_eq(v.x(), x));
        assert!(approx_eq(v.y(), y));
        assert!(approx_eq(v.z(), z));
    }

    #[test_case(2.0, -3.0, 6.0, 49.0         ; "squared length = 49")]
    #[test_case(0.0, 0.0, 0.0, 0.0           ; "squared length of zero = 0")]
    #[test_case(1.0, 1.0, 1.0, 3.0           ; "squared length of (1,1,1) = 3")]
    fn test_squid(x: f64, y: f64, z: f64, expected: f64) {
        let v = Vector3D::new(x, y, z);
        assert!(approx_eq(v.squid(), expected));
    }

    #[test_case(3.0, 4.0, 12.0, 13.0        ; "length = 13")]
    #[test_case(0.0, 0.0, 0.0, 0.0          ; "length of zero vector = 0")]
    #[test_case(1.0, 1.0, 1.0, 3f64.sqrt()  ; "length of (1,1,1) = sqrt(3)")]
    fn test_length(x: f64, y: f64, z: f64, expected: f64) {
        let v = Vector3D::new(x, y, z);
        assert!(approx_eq(v.length(), expected));
    }

    #[test_case(1.0, 2.0, 3.0, -2.0, 0.5, 4.0, 11.0 ; "dot product = 11")]
    #[test_case(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0  ; "orthogonal vectors dot = 0")]
    #[test_case(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 3.0  ; "self dot = squared length")]
    fn test_dot(ax: f64, ay: f64, az: f64, bx: f64, by: f64, bz: f64, expected: f64) {
        let a = Vector3D::new(ax, ay, az);
        let b = Vector3D::new(bx, by, bz);
        assert!(approx_eq(a.dot(&b), expected));
    }

    #[allow(clippy::too_many_arguments)]
    #[test_case(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0; "cross(X,Y)=Z")]
    #[test_case(0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0; "cross(Y,Z)=X")]
    #[test_case(0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0; "cross(Z,X)=Y")]
    fn test_cross(ax: f64, ay: f64, az: f64, bx: f64, by: f64, bz: f64, cx: f64, cy: f64, cz: f64) {
        let a = Vector3D::new(ax, ay, az);
        let b = Vector3D::new(bx, by, bz);
        let c = a.cross(&b);
        assert!(approx_eq(c.x(), cx));
        assert!(approx_eq(c.y(), cy));
        assert!(approx_eq(c.z(), cz));
    }

    #[allow(clippy::too_many_arguments)]
    #[test_case(1.0, 2.0, 3.0, -1.0, 4.0, 0.5, 0.0, 6.0, 3.5 ; "add vectors")]
    #[test_case(0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0 ; "add zero vector")]
    #[test_case(-1.0, -1.0, -1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0 ; "add opposite and equal vector")]
    fn test_add(ax: f64, ay: f64, az: f64, bx: f64, by: f64, bz: f64, sx: f64, sy: f64, sz: f64) {
        let a = Vector3D::new(ax, ay, az);
        let b = Vector3D::new(bx, by, bz);
        let sum = a.add(&b);
        assert!(approx_eq(sum.x(), sx));
        assert!(approx_eq(sum.y(), sy));
        assert!(approx_eq(sum.z(), sz));
    }

    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::too_many_arguments)]
    #[test_case(1.0, 2.0, 3.0, -1.0, 4.0, 0.5, 2.0, -2.0, 2.5 ; "subtract vectors")]
    #[test_case(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0 ; "subtract same")]
    #[test_case(1.0, 1.0, 1.0, -1.0, -1.0, -1.0, 2.0, 2.0, 2.0 ; "subtract opposite and equal vector")]
    fn test_subtract(
        ax: f64,
        ay: f64,
        az: f64,
        bx: f64,
        by: f64,
        bz: f64,
        rx: f64,
        ry: f64,
        rz: f64,
    ) {
        let a = Vector3D::new(ax, ay, az);
        let b = Vector3D::new(bx, by, bz);
        let diff = a.subtract(&b);
        assert!(approx_eq(diff.x(), rx));
        assert!(approx_eq(diff.y(), ry));
        assert!(approx_eq(diff.z(), rz));
    }

    #[test_case(2.0, -4.0, 0.5, 3.0, 6.0, -12.0, 1.5 ; "scale by factor")]
    #[test_case(1.0, 1.0, 1.0, -1.0, -1.0, -1.0, -1.0 ; "scale by negative factor")]
    fn test_scale(vx: f64, vy: f64, vz: f64, factor: f64, sx: f64, sy: f64, sz: f64) {
        let v = Vector3D::new(vx, vy, vz);
        let scaled = v.scale(factor);
        assert!(approx_eq(scaled.x(), sx));
        assert!(approx_eq(scaled.y(), sy));
        assert!(approx_eq(scaled.z(), sz));
    }

    #[test_case(2.0, -4.0, 0.5, 2.0, 1.0, -2.0, 0.25 ; "divide by non-zero")]
    #[test_case(2.0, -4.0, 0.5, 0.5, 4.0, -8.0, 1.0 ; "divide by fraction")]
    #[test_case(2.0, -4.0, 0.5, 0.0, 0.0, 0.0, 0.0 ; "divide by zero yields zero vector")]
    fn test_divide(vx: f64, vy: f64, vz: f64, divisor: f64, rx: f64, ry: f64, rz: f64) {
        let v = Vector3D::new(vx, vy, vz);
        let divided = v.divide(divisor);
        assert!(approx_eq(divided.x(), rx));
        assert!(approx_eq(divided.y(), ry));
        assert!(approx_eq(divided.z(), rz));
    }

    #[test_case(3.0, 4.0, 0.0, 3./5., 4./5., 0.0 ; "unit vector in XY-plane")]
    #[test_case(2.0, 0.0, 0.0, 1.0, 0.0, 0.0 ; "unit vector on x axis")]
    #[test_case(0.0, 5.0, 0.0, 0.0, 1.0, 0.0 ; "unit vector on y axis")]
    #[test_case(0.0, 0.0, -333.0, 0.0, 0.0, -1.0 ; "negative unit vector on z axis")]
    #[test_case(1.0, 1.0, 1.0, 1./3f64.sqrt(), 1./3f64.sqrt(), 1./3f64.sqrt() ; "unit vector on diagonal")]
    fn test_unit(vx: f64, vy: f64, vz: f64, ux: f64, uy: f64, uz: f64) {
        let v = Vector3D::new(vx, vy, vz);
        let unit = v.unit();
        assert!(approx_eq(unit.length(), 1.0));
        assert!(approx_eq(unit.x(), ux));
        assert!(approx_eq(unit.y(), uy));
        assert!(approx_eq(unit.z(), uz));
    }

    #[test_case(2.0, 0.0, -5.0, -2.0, 0.0, 5.0 ; "invert flips all signs")]
    fn test_invert(vx: f64, vy: f64, vz: f64, ix: f64, iy: f64, iz: f64) {
        let v = Vector3D::new(vx, vy, vz);
        let inv = v.invert();
        assert!(approx_eq(inv.x(), ix));
        assert!(approx_eq(inv.y(), iy));
        assert!(approx_eq(inv.z(), iz));
    }

    #[test_case(1.0, 2.0, 3.0, 4.0, -1.0, 5.0 ; "to() yields origin - dest")]
    fn test_to_method(ox: f64, oy: f64, oz: f64, dx: f64, dy: f64, dz: f64) {
        let origin = Vector3D::new(ox, oy, oz);
        let dest = Vector3D::new(dx, dy, dz);
        let via = origin.to(&dest);
        let expected = origin.subtract(&dest);
        assert!(approx_eq(via.x(), expected.x()));
        assert!(approx_eq(via.y(), expected.y()));
        assert!(approx_eq(via.z(), expected.z()));
    }

    #[test_case(0.0, 0.0, 0.0, 1.0, 1.0, 1.0 ; "from() to() yields correct difference")]
    fn test_from_to(ox: f64, oy: f64, oz: f64, dx: f64, dy: f64, dz: f64) {
        let origin = Vector3D::new(ox, oy, oz);
        let dest = Vector3D::new(dx, dy, dz);
        let v = Vector3D::from(&origin).to(&dest);
        assert!(approx_eq(v.x(), dx - ox));
        assert!(approx_eq(v.y(), dy - oy));
        assert!(approx_eq(v.z(), dz - oz));
    }

    #[test]
    fn test_constants_and_display() {
        // X, Y, Z, O
        assert!(approx_eq(X.x(), 1.0));
        assert!(approx_eq(X.y(), 0.0));
        assert!(approx_eq(X.z(), 0.0));
        assert!(approx_eq(Y.x(), 0.0));
        assert!(approx_eq(Y.y(), 1.0));
        assert!(approx_eq(Y.z(), 0.0));
        assert!(approx_eq(Z.x(), 0.0));
        assert!(approx_eq(Z.y(), 0.0));
        assert!(approx_eq(Z.z(), 1.0));
        assert!(approx_eq(O.x(), 0.0));
        assert!(approx_eq(O.y(), 0.0));
        assert!(approx_eq(O.z(), 0.0));

        // Display formatting
        let s = format!("{}", Vector3D::new(1.23, -4.56, 7.89));
        assert!(s.contains("x: 1.23"));
        assert!(s.contains("y: -4.56"));
        assert!(s.contains("z: 7.89"));
    }
}
