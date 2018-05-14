// Result enum: express why an operation failed
//    * Ok(value), operation succeeded with wrapped value return
//    * Err(why), operation failed with wrapped cause return

mod checked {
    // math errors to catch
    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero,
        NonPositiveLogarithm,
        NegativeSquareRoot,
    }

    // type alias
    pub type MathResult = Result<f64, MathError>;

    pub fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(x / y)
        }
    }

    pub fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    pub fn ln(x: f64) -> MathResult {
        if x <= 0.0 {
            Err(MathError::NonPositiveLogarithm)
        } else {
            Ok(x.ln())
        }
    }
}

fn op(x: f64, y: f64) -> f64 {
    match checked::div(x, y) {
        Err(why)  => panic!("{:?}", why),
        Ok(ratio) => match checked::ln(ratio) {
            Err(why)  => panic!("{:?}", why),
            Ok(ln)    => match checked::sqrt(ln) {
                Err(why)  => panic!("{:?}", why),
                Ok(sqrt)  => sqrt,
            },
        },
    }
}

fn main() {
    println!("{}", op(1.0, 10.0));
}