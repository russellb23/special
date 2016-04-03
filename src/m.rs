use libc::{c_double, c_int};

extern {
    pub fn erf(x: c_double) -> c_double;
    pub fn erfc(x: c_double) -> c_double;
    pub fn tgamma(x: c_double) -> c_double;
}

#[cfg(unix)]
extern {
    pub fn lgamma_r(x: c_double, sign: &mut c_int) -> c_double;
}

#[cfg(unix)]
#[inline(always)]
pub unsafe fn lgamma(x: c_double, sign: &mut c_int) -> c_double {
    lgamma_r(x, sign)
}

#[cfg(windows)]
extern {
    pub fn lgamma(x: c_double, sign: &mut c_int) -> c_double;
}

#[cfg(test)]
mod tests {
    use assert;

    #[test]
    fn erf() {
        let x = vec![
            -2.632984211817266e+00, -2.363273199619327e+00, -2.094834420999714e+00,
            -1.827652978390535e+00, -1.561714251393564e+00, -1.297003889738660e+00,
            -1.033507806468575e+00, -7.712121713408920e-01, -5.101034044390804e-01,
            -2.501681699844875e-01,  8.606629658243489e-03,  2.662338597896692e-01,
             5.227261590049689e-01,  7.780959446508342e-01,  1.032355418116290e+00,
             1.285516569963673e+00,  1.537591184905182e+00,  1.788590846631104e+00,
             2.038526942494747e+00,  2.287410668059257e+00,
        ];
        let y = vec![
            4.231914082376952e-03, 9.057155208654155e-03, 1.809285705645609e-02,
            3.380082467503698e-02, 5.917765997943183e-02, 9.731492422005782e-02,
            1.506831595865132e-01, 2.202905905012284e-01, 3.049895101174467e-01,
            4.012286497130861e-01, 5.034335060734141e-01, 6.049704463017676e-01,
            6.994175842035349e-01, 7.817437733581876e-01, 8.490471756668190e-01,
            9.006940849004457e-01, 9.379256979186316e-01, 9.631596334645754e-01,
            9.792513684346750e-01, 9.889140679020433e-01,
        ];

        let z = x.iter().map(|&x| gaussian(x)).collect::<Vec<_>>();
        assert::close(&z, &y, 1e-14);
    }

    fn gaussian(x: f64) -> f64 {
        use std::f64::consts::FRAC_1_SQRT_2;
        0.5 * (1.0 + unsafe { super::erf(FRAC_1_SQRT_2 * x) })
    }
}
