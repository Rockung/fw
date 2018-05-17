// FFI: foreign function interface

use std::fmt;

// links to the libm library
#[link(name = "m")]
extern {
    // square root of a single precision complex number
    fn csqrtf(z: Complex) -> Complex;

    fn ccosf(z: Complex)  -> Complex;
}

fn cos(z: Complex) -> Complex {
    // unsafe to call a foreign function
    unsafe { ccosf(z) }
}

fn main() {
    let z = Complex { re: -1., im: 0. };
    let z_sqrt = unsafe { csqrtf(z) };

    println!("the square root of {:?} is {:?}", z, z_sqrt);
    println!("cos({:?}) = {:?}", z, cos(z));
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Complex {
    re: f32,
    im: f32,
}

impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.im < 0. {
            write!(f, "{}-{}i", self.re, -self.im)
        } else {
            write!(f, "{}+{}i", self.re, self.im)
        }
    }
}
