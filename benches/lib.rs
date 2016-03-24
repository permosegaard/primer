// http://www.codeproject.com/Articles/691200/Primality-test-algorithms-Prime-test-The-fastest-w
// https://github.com/danaj/Math-Prime-Util-GMP/blob/17b83d60a2f9bffe14c9116d2bde920e7bee46a0/gmp_main.c

// TODO: 'skip' usage needs more testing

#![feature(test)]
extern crate test;

extern crate primer;

#[cfg(test)]
mod tests {
    #[bench]
    fn naive_zero_to_one_thousand( bencher: &mut super::test::Bencher ) {
        bencher.iter(|| { return super::primer::primes_naive().take( super::test::black_box( 10000 ) ); });
    }

    #[bench]
    fn naive_one_million_to_two_million( bencher: &mut super::test::Bencher ) {
        let count = super::test::black_box( 1000000 );
        bencher.iter(|| { return super::primer::primes_naive().skip( count ).take( count ); });
    }
    
    #[bench]
    fn sieve_of_eratosthenes_zero_to_one_thousand( bencher: &mut super::test::Bencher ) {
        bencher.iter(|| { return super::primer::primes_sieve_of_eratosthenes().take( super::test::black_box( 10000 ) ); });
    }

    #[bench]
    fn sieve_of_eratosthenes_one_million_to_two_million( bencher: &mut super::test::Bencher ) {
        let count = super::test::black_box( 1000000 );
        bencher.iter(|| { return super::primer::primes_sieve_of_eratosthenes().skip( count ).take( count ); });
    }
    
    #[bench]
    fn gpm_zero_to_one_thousand( bencher: &mut super::test::Bencher ) {
        bencher.iter(|| { return super::primer::primes_gpm().take( super::test::black_box( 10000 ) ); });
    }

    #[bench]
    fn gpm_one_million_to_two_million( bencher: &mut super::test::Bencher ) {
        let count = super::test::black_box( 1000000 );
        bencher.iter(|| { return super::primer::primes_gpm().skip( count ).take( count ); });
    }
}
