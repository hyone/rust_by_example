mod checked {
    use std::error::Error;
    use std::fmt;

    #[derive(Debug)]
    enum MathError {
        DivisionByZero,
        NegativeLogarithm,
        NegativeSquareRoot,
    }

    type MathResult = Result<f64, MathError>;

    impl fmt::Display for MathError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.description().fmt(f)
        }
    }

    impl Error for MathError {
        fn description(&self) -> &str {
            match *self {
                MathError::NegativeLogarithm
                    => "logarithm of negative number",
                MathError::DivisionByZero
                    => "division by zero",
                MathError::NegativeSquareRoot
                    => "square root of negative number",
            }
        }

        fn cause(&self) -> Option<&Error> {
            match *self {
                _ => None
            }
        }
    }

    fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(x / y)
        }
    }

    fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    fn ln(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeLogarithm)
        } else {
            Ok(x.ln())
        }
    }

    fn op_(x: f64, y: f64) -> Result<f64, Box<Error>> {
        let ratio = try!(div(x, y));
        let ln    = try!(ln(ratio));
        let sqrt  = try!(sqrt(ln));
        Ok(sqrt)
    }

    pub fn op(x: f64, y: f64) {
        match op_(x, y) {
            Ok(value) => println!("{}", value),
            Err(e)    => println!("Error: {}", e),
        }
    }
}

fn main() {
    checked::op(1.0, 10.0);
}
