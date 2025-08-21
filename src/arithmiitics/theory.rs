pub fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a } else { gcd(b, a % b) }
}



pub struct Number{
    x : i32,
    y : i32,
}

impl Number{
    fn __gcd(&self) -> i32{
        gcd(self.x , self.y)
    }   
}