fn main() {
    ModInt::set_modulus(10);
   
    let a = ModInt::new(7);
    let b = ModInt::new(3);

    println!("a = {}", a);         // 7
    println!("b = {}", b);         // 3
    println!("a + b = {}", a + b); // 0
    println!("a - b = {}", a - b); // 4
    println!("a * b = {}", a * b); // 1
    println!("a / b = {}", a / b); // 9 (7 * inv(3) = 7 * 7 = 49 % 10 = 9)

    println!("a.pow(3) = {}", a.pow(3)); // 343 % 10 = 3

    println!("b.inv() = {}", b.inv());     // 7
    println!("b * b.inv() = {}", b * b.inv()); // 1

 
    let c = ModInt::new(-14);
    let d = ModInt::new(29u32);
    println!("c  = {}", c); // 6
    println!("d = {}", d); // 9

    let e: ModInt = "23".parse().unwrap();
    println!("e  = {}", e); // 3

}


//ModInt
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

    pub fn new<T>(value: T) -> Self
    where
         T: TryInto<i64, Error: std::fmt::Debug>
    {
        let modulus = Self::modulus();
        ModInt {
            val: value.try_into().unwrap().rem_euclid(modulus as i64) as usize,
        }
    }

    pub fn inv(self) -> Self {
        let m = Self::modulus() as i64;
        let a = self.val as i64;
        let (g, x, _) = Self::extended_gcd(a, m);
        assert!(g == 1, "Inverse does not exist as GCD ≠ 1");
        ModInt::new((x.rem_euclid(m)) as usize)
    }

    fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
        if b == 0 {
            (a, 1, 0)
        } else {
            let (g, x, y) = Self::extended_gcd(b, a % b);
            (g, y, x - (a / b) * y)
        }
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
