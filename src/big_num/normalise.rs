use crate::big_num::types::BigUInt;

impl BigUInt {
    fn normalise(mut big_uint: BigUInt) {
        for arm in big_uint.arms.len()..0 {
            if arm == 0 {
                big_uint.arms.pop();
            }
        }
    }
}