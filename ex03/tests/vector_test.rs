use rust_linear_algebra::vector::Vector;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dot_product_zero() {
        let u = Vector::from([0.0, 0.0]);
        let v = Vector::from([1.0, 1.0]);

        assert_eq!(u.dot(v), 0.0);
    }

    #[test]
    fn dot_product_two() {
        let u = Vector::from([1.0, 1.0]);
        let v = Vector::from([1.0, 1.0]);

        assert_eq!(u.dot(v), 2.0);
    }

    #[test]
    fn dot_product_nine() {
        let u = Vector::from([-1.0, 6.0]);
        let v = Vector::from([3.0, 2.0]);

        assert_eq!(u.dot(v), 9.0);
    }

    #[test]
    fn dot_symbole_product_zero() {
        let u = Vector::from([0.0, 0.0]);
        let v = Vector::from([1.0, 1.0]);

        assert_eq!(u * v, 0.0);
    }

    #[test]
    fn dot_symbole_product_two() {
        let u = Vector::from([1.0, 1.0]);
        let v = Vector::from([1.0, 1.0]);

        assert_eq!(u * v, 2.0);
    }

    #[test]
    fn dot_symbole_product_nine() {
        let u = Vector::from([-1.0, 6.0]);
        let v = Vector::from([3.0, 2.0]);

        assert_eq!(u * v, 9.0);
    }

    #[test]
    #[should_panic]
    fn dot_panic() {
        let u = Vector::from([-1.0, 6.0]);
        let v = Vector::from([3.0, 2.0, 3.2]);

        assert_eq!(u.dot(v), 9.0);
    }

    #[test]
    #[should_panic]
    fn dot_symbole_panic() {
        let u = Vector::from([-1.0, 6.0]);
        let v = Vector::from([3.0, 2.0, 3.2]);

        assert_eq!(u * v, 9.0);
    }
}
