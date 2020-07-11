pub fn is_prime(n: usize) -> bool {
    if n == 0 {
        return false;
    } else if n == 1 {
        return false;
    } else if n == 2 {
        return true;
    }

    if n % 2 == 0 {
        return false;
    }

    for i in 3..n {
        if n < i * i {
            break;
        }
        if n % i == 0 {
            return false;
        }
    }
    true
}

#[test]
fn is_prime_test() {
    let res1 = is_prime(0);
    assert_eq!(res1, false);
    let res2 = is_prime(2);
    assert_eq!(res2, true);
    let res3 = is_prime(20);
    assert_eq!(res3, false);
    let res4 = is_prime(43);
    assert_eq!(res4, true);
}

pub fn prime_list(n: usize) -> Vec<usize> {
    let mut ret = vec![];
    if n == 0 || n == 1 {
        return ret;
    }
    let mut v = vec![true; n + 1];
    v[0] = false;
    v[1] = false;
    for i in 2..=n {
        if v[i] == true {
            ret.push(i);
            let mut j = i * 2;
            while j <= n {
                v[j] = false;
                j += i;
            }
        }
    }
    ret
}

#[test]
fn prime_list_test() {
    let res1 = prime_list(0);
    assert_eq!(res1, &[]);
    let res2 = prime_list(2);
    assert_eq!(res2, &[2]);
    let res3 = prime_list(5);
    assert_eq!(res3, &[2, 3, 5]);
    let res4 = prime_list(6);
    assert_eq!(res4, &[2, 3, 5]);
    let res5 = prime_list(7);
    assert_eq!(res5, &[2, 3, 5, 7]);
}
