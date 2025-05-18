use statrs::distribution::{Discrete, Poisson};
use statrs::prec;
use statrs::statistics::Distribution;

fn main() {
    let n = Poisson::new(1.0).unwrap();
    assert_eq!(n.mean().unwrap(), 1.0);
    assert!(prec::almost_eq(n.pmf(1), 0.367879441171442, 1e-15));
    println!("Tests ran");
}
