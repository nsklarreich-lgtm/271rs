#[macro_export]
macro_rules! choices {
($a:expr, $b:expr, $c:expr)=> {
dbg!(size_of_val(&1));
}
}
macro_rules! choice {
    ($cond:expr, $x:expr, $y:expr) => {{
        let c = $cond;
        let a = $x;
        let b = $y;
    
        (c & a) | ((!c) & b)
    }};
}
macro_rules! majority {
    ($a:expr, $b:expr, $c:expr) => {{
        let a = $a;
        let b = $b;
        let c = $c;
    
        (a & b) | (a & c) | (b & c)
    }};
}
macro_rules! rotate_bits {
    ($val:expr, $shift:expr) => {{
        let v = $val;
        let s = $shift;
        if s >= 0 {
            v.rotate_left(s as u32)
        } else {
            v.rotate_right((-s) as u32)
        }
    }};
}
fn main() {
    println!("Hello, world!");
    let cond = 0b1100u8;
    let x =    0b1111u8;
    let y =    0b0000u8;

    let result = choice!(cond, x, y);
    println!("{:08b}", result); // 11110000

}
