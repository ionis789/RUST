fn add_space(s: &mut String, n: usize) {
    for _ in 0..n {
        s.push(' ');
    }
}

fn add_str(s: &mut String, slice: &str) {
    for ch in slice.chars() {
        s.push(ch);
    }
}

fn add_integer(s: &mut String, mut n: u64) {
    let mut digits = [0u8; 20];
    let mut len = 0;

    while n > 0 {
        digits[len] = (n % 10) as u8;
        n /= 10;
        len += 1;
    }

    if len == 0 {
        s.push('0');
        return;
    }

    for i in (0..len).rev() {
        s.push((digits[i] + b'0') as char);
        if i % 3 == 0 && i != 0 {
            s.push(',');
        }
    }
}

fn add_float(s: &mut String, f: f64) {
    let int_part = f as u64;
    let frac_part = ((f - int_part as f64) * 10000.0) as u64;

    add_integer(s, int_part);
    s.push('.');

    let mut mult = 1000;
    for _ in 0..4 {
        let digit = (frac_part / mult) % 10;
        s.push((digit as u8 + b'0') as char);
        mult /= 10;
    }
}

fn main() {
    let mut s = String::new();

    add_str(&mut s, "Hello");
    add_space(&mut s, 1);
    add_str(&mut s, "World");
    add_space(&mut s, 1);
    add_integer(&mut s, 123456);
    add_space(&mut s, 1);
    add_str(&mut s, "Pi");
    add_space(&mut s, 3);
    add_str(&mut s, "=");
    add_space(&mut s, 1);
    add_float(&mut s, 3.1415);

    println!("{}", s);
}