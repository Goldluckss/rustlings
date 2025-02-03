struct ColorRegularStruct {
    // TODO: Add the fields that the test `regular_structs` expects.
    // What types should the fields have? What are the minimum and maximum values for RGB colors?

    red: u8,
    green: u8,
    blue: u8,
}

struct ColorTupleStruct(u8, u8, u8);


#[derive(Debug)]
struct UnitStruct;

fn main() {
    // Instantiate and print ColorRegularStruct
    let regular_color = ColorRegularStruct {
        red: 0,
        green: 255,
        blue: 0,
    };
    println!("ColorRegularStruct: red={}, green={}, blue={}", regular_color.red, regular_color.green, regular_color.blue);

    // Instantiate and print ColorTupleStruct
    let tuple_color = ColorTupleStruct(0, 255, 0);
    println!("ColorTupleStruct: red={}, green={}, blue={}", tuple_color.0, tuple_color.1, tuple_color.2);

    // Instantiate and print UnitStruct
    let unit = UnitStruct;
    println!("UnitStruct: {:?}", unit);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        // TODO: Instantiate a regular struct.
        // let green =

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
        // TODO: Instantiate a tuple struct.
        // let green =

        let green = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct.
        // let unit_struct =
        let unit_struct = UnitStruct;

        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }
}
