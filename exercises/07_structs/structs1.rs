// Regular struct with named fields
struct ColorRegularStruct {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple struct: unnamed fields but same idea
struct ColorTupleStruct(u8, u8, u8);

// Unit struct: zero-size type, like a tag
#[derive(Debug)]
struct UnitStruct;

fn main() {
    // You can experiment here if needed.
}

#[cfg(test)]
mod tests {
    use super::*; // Bring struct definitions into scope

    #[test]
    fn regular_structs() {
        // Instantiate a regular struct (with named fields)
        let green = ColorRegularStruct {
            red: 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // Instantiate a tuple struct
        let green = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // Instantiate a unit struct
        let unit_struct = UnitStruct;

        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }
}
