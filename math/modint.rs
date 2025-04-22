#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ModInt {
    val: usize,
}

pub static MOD: std::sync::OnceLock<usize> = std::sync::OnceLock::new();

impl ModInt {
    
    pub fn set_modulus(modulus: usize) {
        MOD.set(modulus).unwrap_or_else(|_| panic!("Modulus can be set only once"));
    }

    fn modulus() -> usize {
        *MOD.get().expect("Modulus is not set")
    }

    pub fn new(value: usize) -> Self {
        let modulus = Self::modulus();
        ModInt {
            val: value % modulus,
        }
    }

    pub fn inv(self) -> Self {
        self.pow(Self::modulus() - 2)
    }

    pub fn pow(self, exp: usize) -> Self {
        let modulus = Self::modulus();
        let mut base = self.val;
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
}

impl std::ops::Add for ModInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        ModInt::new(self.val + rhs.val)
    }
}

impl std::ops::Sub for ModInt {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let modulus = Self::modulus();
        ModInt::new(self.val + modulus - rhs.val)
    }
}

impl std::ops::Mul for ModInt {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        ModInt::new(self.val * rhs.val)
    }
}

impl std::ops::Div for ModInt {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        ModInt::new(self.val * rhs.inv().val)
    }
}

impl std::ops::AddAssign for ModInt {
    fn add_assign(&mut self, rhs: Self) {
        let modulus = Self::modulus();
        self.val = (self.val + rhs.val) % modulus;
    }
}

impl std::ops::SubAssign for ModInt {
    fn sub_assign(&mut self, rhs: Self) {
        let modulus = Self::modulus();
        self.val = (self.val + modulus - rhs.val) % modulus;
    }
}

impl std::ops::MulAssign for ModInt {
    fn mul_assign(&mut self, rhs: Self) {
        let modulus = Self::modulus();
        self.val = (self.val * rhs.val) % modulus;
    }
}

impl std::ops::DivAssign for ModInt {
    fn div_assign(&mut self, rhs: Self) {
        let modulus = Self::modulus();
        self.val = (self.val * rhs.inv().val) % modulus;
    }
}

impl std::fmt::Display for ModInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::write!(f, "{}", self.val)
    }
}

impl std::str::FromStr for ModInt {
    type Err = std::string::String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parsed_val = s
            .trim()
            .parse::<usize>()
            .map_err(|e| std::format!("Failed to parse ModInt from string: {}", e))?;
        std::result::Result::Ok(ModInt::new(parsed_val))
    }
}


