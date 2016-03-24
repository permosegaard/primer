// http://www.codeproject.com/Articles/691200/Primality-test-algorithms-Prime-test-The-fastest-w
// https://github.com/danaj/Math-Prime-Util-GMP/blob/17b83d60a2f9bffe14c9116d2bde920e7bee46a0/gmp_main.c
// https://gist.github.com/jsanders/8739134
// https://rust-num.github.io/num/num/index.html

extern crate gmp;


fn rough_root( number: &gmp::mpz::Mpz ) -> gmp::mpz::Mpz { return number.sqrt(); }

fn divisible_by_x( number: &gmp::mpz::Mpz, x: &gmp::mpz::Mpz ) -> bool {
    if number % x == std::convert::From::<i64>::from( 0 ) { return true; }
    else { return false; }
}
fn divisible_by_two( number: &gmp::mpz::Mpz ) -> bool { return divisible_by_x( number, &From::<i64>::from( 2 ) ); } // test last bit??


pub enum PrimeTesters { Naive, SieveOfEratosthenes, GPM }

fn naive_primality( number: &gmp::mpz::Mpz ) -> bool {
    if *number == std::convert::From::<i64>::from( 0 ) || *number == std::convert::From::<i64>::from( 1 ) { return false; }
    else if *number == std::convert::From::<i64>::from( 2 ) { return true; }

    // replace with augmented smart_increment
    if divisible_by_two( &number ) { return false; } 
    
    let root = rough_root( number );
    
    let mut current : gmp::mpz::Mpz = std::convert::From::<i64>::from( 3 );
    while current <= root {

        match divisible_by_x( &number, &current ) {

            true => return false,
            false => current = &current + 2 // should probably be replaced by smart increment

        }

    }
    
    return true;
}

fn sieve_of_eratosthenes_primality( number: &gmp::mpz::Mpz ) -> bool {
    if *number == std::convert::From::<i64>::from( 0 ) || *number == std::convert::From::<i64>::from( 1 ) { return false; }
    else if *number == std::convert::From::<i64>::from( 2 ) { return true; }
    
    let mut current : gmp::mpz::Mpz = std::convert::From::<i64>::from( 0 );
    while current < *number {
    
        let mut multiplier : gmp::mpz::Mpz = std::convert::From::<i64>::from( 0 );
        while multiplier < *number {

            if &current * &multiplier == *number { return false; }
            else if &current * &multiplier > *number { break; }
            else { multiplier = &multiplier + 1; }

        }

        current = &current + 1;

    }

    return true;
}

fn gpm_primality( number: &gmp::mpz::Mpz ) -> bool {
    if *number == std::convert::From::<i64>::from( 0 ) || *number == std::convert::From::<i64>::from( 1 ) { return false; }
    else if *number == std::convert::From::<i64>::from( 2 ) { return true; }
    
    let previous_number = &number.clone() - 1;

    if previous_number.nextprime() == *number { return true; }
    else { return false; }
}

fn is_prime( number: &gmp::mpz::Mpz, tester: &PrimeTesters ) -> bool {
    match tester {
        &PrimeTesters::Naive => return naive_primality( number ),
        &PrimeTesters::SieveOfEratosthenes => return sieve_of_eratosthenes_primality( number ),
        &PrimeTesters::GPM => return gpm_primality( number ),
    }
}

pub struct Primes { testing: gmp::mpz::Mpz, tester: PrimeTesters }
impl Primes {
    fn test( &mut self ) -> bool { return is_prime( &self.testing, &self.tester ); }
    fn smart_increment( &mut self ) {
        if self.testing == std::convert::From::<i64>::from( 0 ) ||
           self.testing == std::convert::From::<i64>::from( 1 ) ||
           self.testing == std::convert::From::<i64>::from( 2 ) { self.testing = &self.testing + 1; }
        else { self.testing = &self.testing + 2; }
    }
    fn increment( &mut self ) { self.smart_increment(); }
    
    pub fn get_testing( &mut self ) -> gmp::mpz::Mpz { return self.testing.clone(); }
    pub fn set_testing( &mut self, testing: &gmp::mpz::Mpz ) { self.testing = testing.clone(); }
    pub fn test_and_increment( &mut self ) -> Option<gmp::mpz::Mpz> {
        match self.test() {
            true => { let prime = self.testing.clone(); self.increment(); return Some( prime ); },
            false => { self.increment(); return None; }
        }
    }
}
impl Iterator for Primes {
    type Item = gmp::mpz::Mpz;
    fn next( &mut self ) -> Option<gmp::mpz::Mpz> {
        loop {
            match self.test_and_increment() {
                Some( prime ) => return Some( prime ),
                None => {}
            }
        }
    }
//  fn skip( &mut self ) blah { self.current = arg } // speed optimisation, set to absolute number?!
}

fn primes( tester: PrimeTesters ) -> Primes { return Primes { testing: std::convert::From::<i64>::from( 0 ), tester: tester }; }
pub fn primes_naive() -> Primes { return primes( PrimeTesters::Naive ); }
pub fn primes_sieve_of_eratosthenes() -> Primes { return primes( PrimeTesters::SieveOfEratosthenes ); }
pub fn primes_gpm() -> Primes { return primes( PrimeTesters::GPM ); }
