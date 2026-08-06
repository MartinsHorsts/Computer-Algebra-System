#[cfg(test)]
mod tests {
    use crate::big_num::types::*;


    #[test]
    fn normalise_zero_match() {
        let expected = BigInt { sign: Sign::Zero, data: BigUInt{arms: vec!(0)}};
        let mut num1 = BigInt { sign: Sign::Zero, data: BigUInt{arms: vec!(0)}};
        num1.data = num1.data.normalise();

        assert_eq!(num1,expected)
    }

    #[test]
    fn normalise_non_leading_zero () {
        let expected = BigInt { sign: Sign::Positive, data: BigUInt{arms: vec!(5,0,5)}};
        let mut num1 = BigInt { sign: Sign::Positive, data: BigUInt{arms: vec!(5,0,5)}};
        num1.data = num1.data.normalise();
        assert_eq!(num1, expected)
    }

    #[test]
    fn normalise_leading_zeros () {
        let expected = BigInt { sign: Sign::Positive, data: BigUInt{arms: vec!(5)}};
        let mut num1 = BigInt { sign: Sign::Positive, data: BigUInt{arms: vec!(5,0,0)}};
        num1.data = num1.data.normalise();
        assert_eq!(num1,expected)
    }

    #[test]
    fn big_int_add_no_carry () {
        let num1 = BigInt { sign: Sign::Positive, data: BigUInt{arms: vec!(5)}};
        let num2 = BigInt { sign: Sign::Positive, data: BigUInt{arms: vec!(5)}};
        let sum = num1+num2;

        let expected = BigInt { sign: Sign::Positive, data: BigUInt{arms: vec!(10)}};

        assert_eq!(sum, expected)
    }

    #[test]
    fn big_int_add_carry () {
        let num1 = BigInt { sign: Sign::Positive, data: BigUInt{arms: vec!(9223372036854775809)}};
        let num2 = BigInt { sign: Sign::Positive, data: BigUInt{arms: vec!(9223372036854775808)}};
        let sum = num1 + num2;

        let expected = BigInt { sign: Sign::Positive, data: BigUInt{arms: vec!(1,1)}};

        assert_eq!(sum, expected)
    }

    #[test]
    fn big_int_sub_equal () {
        let num1 = BigInt { sign: Sign::Positive, data: BigUInt{arms: vec!(5)}};
        let num2 = BigInt { sign: Sign::Positive, data: BigUInt{arms: vec!(5)}};
        let sum = num1-num2;

        let expected = BigInt { sign: Sign::Zero, data: BigUInt{arms: vec!(0)}};

        assert_eq!(sum, expected)
    }

    #[test]
    fn big_int_sub_no_carry_no_sign_change () {
        let num1 = BigInt { sign: Sign::Positive, data: BigUInt{arms: vec!(7)}};
        let num2 = BigInt { sign: Sign::Positive, data: BigUInt{arms: vec!(5)}};
        let sum = num1-num2;

        let expected = BigInt { sign: Sign::Positive, data: BigUInt{arms: vec!(2)}};

        assert_eq!(sum, expected)
    }

    #[test]
    fn big_int_sub_carry_no_sign_change () {
        let num1 = BigInt { sign: Sign::Positive, data: BigUInt{arms: vec!(1,1)}};
        let num2 = BigInt { sign: Sign::Positive, data: BigUInt{arms: vec!(9223372036854775809)}};
        let sum = num1-num2;

        let expected = BigInt { sign: Sign::Positive, data: BigUInt{arms: vec!(9223372036854775808)}};

        assert_eq!(sum, expected)
    }
}