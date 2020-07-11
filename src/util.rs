pub fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let r = b;
        b = a % b;
        a = r;
    }
    a
}

#[test]
fn gcd_test() {
    let res1 = gcd(12, 24);
    assert_eq!(res1, 12);
    let res2 = gcd(24, 12);
    assert_eq!(res2, 12);
    let res3 = gcd(611, 2679);
    assert_eq!(res3, 47);
}

pub fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

#[test]
fn lcm_test() {
    let res1 = lcm(3, 5);
    assert_eq!(res1, 15);
    let res2 = lcm(15, 24);
    assert_eq!(res2, 120);
}

pub fn mod_pow(x: usize, n: usize, m: usize) -> usize {
    if n == 0 {
        return 1;
    }
    let mut ans = 1;
    let mut x = x;
    let mut n = n;
    while 0 < n {
        if n % 2 == 1 {
            ans = (ans * x) % m;
        }
        x = (x * x) % m;
        n >>= 1;
    }
    ans
}

#[test]
fn mod_pow_test() {
    let res1 = mod_pow(2, 3, 1000000007);
    assert_eq!(res1, 8);
    let res2 = mod_pow(3, 0, 1000000007);
    assert_eq!(res2, 1);
}

pub fn mod_inv(x: usize, m: usize) -> usize {
    mod_pow(x, m - 2, m)
}

#[test]
fn mod_inv_test() {
    let res1 = mod_inv(3, 11);
    assert_eq!(res1, 4);
    let res2 = mod_inv(5, 13);
    assert_eq!(res2, 8);
}

pub fn ncr(n: usize, r: usize, m: usize) -> usize {
    let r = std::cmp::min(r, n - r);
    let mut numerator = 1;
    let mut denominator = 1;
    for i in 0..r {
        numerator = (numerator * (n - i)) % m;
        denominator = (denominator * (i + 1)) % m;
    }
    numerator * mod_inv(denominator, m) % m
}

#[test]
fn ncr_test() {
    let res1 = ncr(5, 3, 1000000007);
    assert_eq!(res1, 10);
    let res2 = ncr(50, 49, 1000000007);
    assert_eq!(res2, 50);
}
