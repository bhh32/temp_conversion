pub mod from_celsisus {
    const nine_fifths_float: f64 = 9.0/5.0;
    const celsisus_float: f64 = 32.0;
    const kelvin_float: f64 = 273.0;
    const nine_fifths_int: i64 = 9/5;
    const kelvin_int: i64 = 273;
    const celsisus_int: i64 = 32;

    pub struct Celsius;

    pub trait ToFarenheit<T> {
        fn to_fahrenheit(&self, T: T) -> T;
    }

    impl ToFarenheit<f64> for Celsius {
        fn to_fahrenheit(&self, temp_in: f64) -> f64 {
            return (nine_fifths_float * temp_in) + celsisus_float;
        }
    }

    impl ToFarenheit<i64> for Celsius {
        fn to_fahrenheit(&self, temp_in: i64) -> i64 {
            return (nine_fifths_int * temp_in) + celsisus_int;
        }
    }

    pub trait ToKelvin<T> {
        fn to_kelvin(&self, T: T) -> T;
    }

    impl ToKelvin<f64> for Celsius {
        fn to_kelvin(&self, temp_in: f64) -> f64 {
            return temp_in + kelvin_float;
        }
    }
    
    impl ToKelvin<i64> for Celsius {
        fn to_kelvin(&self, temp_in: i64) -> i64 {
            return temp_in + kelvin_int;
        }
    }
}