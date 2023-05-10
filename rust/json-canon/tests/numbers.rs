use json_canon::to_string;
use serde_json::{from_str, Value};

#[test]
fn test_numbers() {
    #[track_caller]
    fn test_json_number(bits: u64, expected: &str) {
        assert_eq!(to_string(&f64::from_bits(bits)).unwrap(), expected);
    }

    // fn test_json_number_err(bits: u64) {
    //   assert!(to_string(&f64::from_bits(bits)).is_err());
    // }

    test_json_number(0x0000000000000000, "0"); // Zero
    test_json_number(0x8000000000000000, "0"); // Minus zero
    test_json_number(0x0000000000000001, "5e-324"); // Min pos number
    test_json_number(0x8000000000000001, "-5e-324"); // Min neg number
    test_json_number(0x7fefffffffffffff, "1.7976931348623157e+308"); // Max pos number
    test_json_number(0xffefffffffffffff, "-1.7976931348623157e+308"); // Max neg number
    test_json_number(0x4340000000000000, "9007199254740992"); // Max pos int
    test_json_number(0xc340000000000000, "-9007199254740992"); // Max neg int
    test_json_number(0x4430000000000000, "295147905179352830000"); // ~2**68

    // TODO: Custom Serializer...
    // test_json_number_err(0x7fffffffffffffff); // NaN
    // test_json_number_err(0x7ff0000000000000); // Infinity

    test_json_number(0x44b52d02c7e14af5, "9.999999999999997e+22");
    test_json_number(0x44b52d02c7e14af6, "1e+23");
    test_json_number(0x44b52d02c7e14af7, "1.0000000000000001e+23");
    test_json_number(0x444b1ae4d6e2ef4e, "999999999999999700000");
    test_json_number(0x444b1ae4d6e2ef4f, "999999999999999900000");
    test_json_number(0x444b1ae4d6e2ef50, "1e+21");
    test_json_number(0x3eb0c6f7a0b5ed8c, "9.999999999999997e-7");
    test_json_number(0x3eb0c6f7a0b5ed8d, "0.000001");
    test_json_number(0x41b3de4355555553, "333333333.3333332");
    test_json_number(0x41b3de4355555554, "333333333.33333325");
    test_json_number(0x41b3de4355555555, "333333333.3333333");
    test_json_number(0x41b3de4355555556, "333333333.3333334");
    test_json_number(0x41b3de4355555557, "333333333.33333343");
    test_json_number(0xbecbf647612f3696, "-0.0000033333333333333333");
    test_json_number(0x43143ff3c1cb0959, "1424953923781206.2"); // Round to even
}
