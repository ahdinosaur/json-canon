use std::{
    env::current_dir,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    process::{Command, Stdio},
    str,
};

use json_canon::to_string;

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

#[test]
fn test_number_data_from_file() -> Result<(), io::Error> {
    let test_data_path = current_dir()?.join(Path::new("../../test-data/generated/numbers.txt"));

    // only run test if generated file exists
    if !test_data_path.exists() {
        return Ok(());
    }

    let file = File::open(test_data_path)?;
    let reader = BufReader::new(file);
    for line_result in reader.lines() {
        let line = line_result?;
        let mut split = line.split(',');
        let bits_str = split.next().ok_or(io::Error::new(
            io::ErrorKind::InvalidData,
            "Test data: `bits` not found",
        ))?;
        let bits = u64::from_str_radix(bits_str, 16).map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Test data: `bits` not parseable to u64",
            )
        })?;
        let expected = split.next().ok_or(io::Error::new(
            io::ErrorKind::InvalidData,
            "Test data: `expected` not found",
        ))?;
        assert_eq!(split.next(), None);
        test_json_number(bits, expected);
    }

    Ok(())
}

#[test]
fn test_data_from_command() -> Result<(), io::Error> {
    let test_command_path = current_dir()?.join(Path::new("../../js/json-canon-fuzz/src/numbers"));

    let mut child = Command::new("node")
        .arg(test_command_path)
        .arg("100000")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

    let stdout = child.stdout.as_mut().expect("Failed to open stdout");
    let reader = io::BufReader::new(stdout);

    for line_result in reader.lines() {
        let line = line_result?;

        let mut split = line.split(',');
        let bits_str = split.next().ok_or(io::Error::new(
            io::ErrorKind::InvalidData,
            "Test data: `bits` not found",
        ))?;
        let bits = u64::from_str_radix(bits_str, 16).map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Test data: `bits` not parseable to u64",
            )
        })?;
        let expected = split.next().ok_or(io::Error::new(
            io::ErrorKind::InvalidData,
            "Test data: `expected` not found",
        ))?;
        assert_eq!(split.next(), None);

        test_json_number(bits, expected);
    }

    let ecode = child.wait()?;
    assert!(ecode.success());

    Ok(())
}

#[track_caller]
fn test_json_number(bits: u64, expected: &str) {
    assert_eq!(to_string(&f64::from_bits(bits)).unwrap(), expected);
}
