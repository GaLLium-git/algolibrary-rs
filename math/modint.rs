pub struct ModInt {
    value: usize,
}

impl ModInt {
    pub static MOD: std::cell::OnceCell<usize> = std::cell::OnceCell::new();

    pub fn set_modulus(modulus: usize) {
        MOD.set(modulus).unwrap_or_else(|_| panic!("Modulus can be set only once"));
    }

    fn modulus() -> usize {
        *MOD.get().expect("Modulus is not set")
    }

    pub fn new(value: usize) -> Self {
        let modulus = Self::modulus();
        ModInt {
            value: value % modulus,
        }
    }

    pub fn inv(self) -> Self {
        let modulus = Self::modulus();
        let inv_value = self.extended_gcd(self.value, modulus).0;
        ModInt::new(inv_value)
    }

    pub fn pow(self, exp: usize) -> Self {
        let modulus = Self::modulus();
        let mut base = self.value;
        let mut result = 1;
        let mut exp = exp;

        while exp > 0 {
            if exp % 2 == 1 {
                result = (result * base) % modulus;
            }
            base = (base * base) % modulus;
            exp /= 2;
        }

        ModInt::new(result)
    }

    fn extended_gcd(&self, a: usize, b: usize) -> (usize, usize, usize) {
        if b == 0 {
            (a, 1, 0)
        } else {
            let (g, x, y) = self.extended_gcd(b, a % b);
            (g, y, x - (a / b) * y)
        }
    }
}

impl std::ops::Add for ModInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let modulus = Self::modulus();
        ModInt::new(self.value + rhs.value)
    }
}

impl std::ops::Sub for ModInt {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let modulus = Self::modulus();
        ModInt::new(self.value + modulus - rhs.value)
    }
}

impl std::ops::Mul for ModInt {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let modulus = Self::modulus();
        ModInt::new(self.value * rhs.value)
    }
}

impl std::ops::Div for ModInt {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let modulus = Self::modulus();
        ModInt::new(self.value * rhs.inv().value)
    }
}

impl std::ops::AddAssign for ModInt {
    fn add_assign(&mut self, rhs: Self) {
        let modulus = Self::modulus();
        self.value = (self.value + rhs.value) % modulus;
    }
}

impl std::ops::SubAssign for ModInt {
    fn sub_assign(&mut self, rhs: Self) {
        let modulus = Self::modulus();
        self.value = (self.value + modulus - rhs.value) % modulus;
    }
}

impl std::ops::MulAssign for ModInt {
    fn mul_assign(&mut self, rhs: Self) {
        let modulus = Self::modulus();
        self.value = (self.value * rhs.value) % modulus;
    }
}

impl std::ops::DivAssign for ModInt {
    fn div_assign(&mut self, rhs: Self) {
        let modulus = Self::modulus();
        self.value = (self.value * rhs.inv().value) % modulus;
    }
}
