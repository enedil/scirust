/// This module provides useful discrete mathematics functions

// std imports

// external imports
use num::traits::Signed;
use std::collections::HashMap;
// local imports


#[inline]
pub fn mod_n (x : isize, n : isize) -> usize {
    let x = x % n;
    if x < 0{
        (x + n.abs()) as usize
    }else{
        x as usize
    }
}

pub fn modexp(base: u64, exp: u64, modulus: u64) -> u64 {
    let mut ret = 1u64;
    let mut e = exp;

    while e != 0 {
	if e%2 == 0 {
            ret *= ret; 
            e /= 2; 
        } else {
            ret *= base;
            e -= 1;
        }
        ret %= modulus;
    }
    ret
}



pub fn order_n(a: u64, n: u64) -> u64 {
    let mut aa = 1;
    for i in 1..n {
        aa *= a;
        aa %= n;
        if aa == 1 {
            return i;
        }
    }
}

pub fn vp (p: u64, n: u64) -> u64 {
	if n%p == 0 {
		return 1 + vp(p, n/p);
	}
	0u64
}

fn gcd(a: u64, b: u64) -> u64 {
	let aa = max(a, b);
	let bb = min(a, b);
	if aa == bb {
		return aa
	}
	if aa%bb == 0 {
		return bb;
	}
	gcd(aa%bb, bb)
}

fn factor_int(n: u64) -> HashMap<u64, u64> {
	let mut factors = HashMap::new();
	let mut nn = n;
	let mut p = 2u64;
	while p*p <= nn {
		while nn%p == 0 {
			let exp = factors.entry(p).or_insert(0);
    			*exp += 1;
			nn /= p;
		}
		p += 1;
	}
	if nn != 1 {
		factors.insert(nn, 1);
	}
	factors
}

fn totient(n: u64) -> u64 {
	let p_divs = factorint(n);
	p_divs.keys().fold(n, |n, p| (n*(p-1))/p)
}


/******************************************************
 *
 *   Unit tests follow.
 *
 *******************************************************/


#[cfg(test)]
mod test{
    #[test]
    fn test_modulo_n(){
        assert_eq!(super::mod_n(4, 5), 4);
        assert_eq!(super::mod_n(6, 5), 1);
        assert_eq!(super::mod_n(-3, 5), 2);
        assert_eq!(super::mod_n(-5, -5), 0);
        assert_eq!(super::mod_n(-6, -5), 4);
    }
    #[test]
    fn test_modexp(){
        assert_eq!(super::modexp(2, 5, 13), 8);
    }
}
