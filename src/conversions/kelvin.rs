pub mod from_kelvin {
    const CELSISUS_CONV_FLOAT: f64 = 1.8;
    const KELVIN_FLOAT: f64 = 273.0;
    const CELSISUS_CONV_INT: i64 = 1.8 as i64;
    const KELVIN_INT: i64 = 273;
    const CELSISUS_FLOAT: f64 = 32.0;
    const CELSISUS_INT: i64 = 32;

    struct Kelvin;

    trait ToCelsisus<T> {
        fn to_celsisus(&self, temp_in: T) -> T;
    }

    impl ToCelsisus<f64> for Kelvin {
        fn to_celsisus(&self, temp_in: f64) -> f64 {
            return temp_in - KELVIN_FLOAT;
        }
    }

    impl ToCelsisus<f32> for Kelvin {
        fn to_celsisus(&self, temp_in: f32) -> f32 {
            return temp_in - KELVIN_FLOAT as f32;
        }
    }

    impl ToCelsisus<i64> for Kelvin {
        fn to_celsisus(&self, temp_in: i64) -> i64 {
            return temp_in - KELVIN_INT;
        }
    }

    impl ToCelsisus<i32> for Kelvin {
        fn to_celsisus(&self, temp_in: i32) -> i32 {
            return temp_in - KELVIN_INT as i32;
        }
    }

    trait ToFarenheit<T> {
        fn to_fahrenheit(&self, temp_in: T) -> T;
    }

    impl ToFarenheit<f64> for Kelvin {
        fn to_fahrenheit(&self, temp_in: f64) -> f64 {
            return (CELSISUS_CONV_FLOAT * Kelvin.to_celsisus(temp_in)) + CELSISUS_FLOAT;
        }
    }

    impl ToFarenheit<f32> for Kelvin {
        fn to_fahrenheit(&self, temp_in: f32) -> f32 {
            return (CELSISUS_CONV_FLOAT as f32 * Kelvin.to_celsisus(temp_in)) + CELSISUS_FLOAT as f32;
        }
    }

    impl ToFarenheit<i64> for Kelvin {
        fn to_fahrenheit(&self, temp_in: i64) -> i64 {
            return (CELSISUS_CONV_INT * Kelvin.to_celsisus(temp_in)) + CELSISUS_INT;
        }
    }

    impl ToFarenheit<i32> for Kelvin {
        fn to_fahrenheit(&self, temp_in: i32) -> i32 {
            return (CELSISUS_CONV_INT as i32 * Kelvin.to_celsisus(temp_in)) + CELSISUS_INT as i32;
        }
    }
}