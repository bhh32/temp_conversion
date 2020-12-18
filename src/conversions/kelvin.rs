pub mod from_kelvin {
    const nine_fifths_float: f64 = 1.8;
    const kelvin_float: f64 = 273.0;
    const nine_fifths_int: i64 = 1.8 as i64;
    const kelvin_int: i64 = 273;
    const celsisus_float: f64 = 32.0;
    const celsisus_int: i64 = 32;

    struct Kelvin;

    trait ToCelsisus<T> {
        fn to_celsisus(&self, T: T) -> T;
    }

    impl ToCelsisus<f64> for Kelvin {
        fn to_celsisus(&self, temp_in: f64) -> f64 {
            return temp_in - kelvin_float;
        }
    }

    impl ToCelsisus<f32> for Kelvin {
        fn to_celsisus(&self, temp_in: f32) -> f32 {
            return temp_in - kelvin_float as f32;
        }
    }

    impl ToCelsisus<i64> for Kelvin {
        fn to_celsisus(&self, temp_in: i64) -> i64 {
            return temp_in - kelvin_int;
        }
    }

    impl ToCelsisus<i32> for Kelvin {
        fn to_celsisus(&self, temp_in: i32) -> i32 {
            return temp_in - kelvin_int as i32;
        }
    }

    trait ToFarenheit<T> {
        fn to_fahrenheit(&self, T: T) -> T;
    }

    impl ToFarenheit<f64> for Kelvin {
        fn to_fahrenheit(&self, temp_in: f64) -> f64 {
            return (nine_fifths_float * Kelvin.to_celsisus(temp_in)) + celsisus_float;
        }
    }

    impl ToFarenheit<f32> for Kelvin {
        fn to_fahrenheit(&self, temp_in: f32) -> f32 {
            return (nine_fifths_float as f32 * Kelvin.to_celsisus(temp_in)) + celsisus_float as f32;
        }
    }

    impl ToFarenheit<i64> for Kelvin {
        fn to_fahrenheit(&self, temp_in: i64) -> i64 {
            return (nine_fifths_int * Kelvin.to_celsisus(temp_in)) + celsisus_int;
        }
    }

    impl ToFarenheit<i32> for Kelvin {
        fn to_fahrenheit(&self, temp_in: i32) -> i32 {
            return (nine_fifths_int as i32 * Kelvin.to_celsisus(temp_in)) + celsisus_int as i32;
        }
    }
}