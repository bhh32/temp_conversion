pub mod from_celsisus {
    pub use crate::conversions::temp_units::units;

    const CELSISUS_CONV_FLOAT: f64 = 1.8;
    const CELSISUS_FLOAT: f64 = 32.0;
    const KELVIN_FLOAT: f64 = 273.0;
    const CELSISUS_CONV_INT: i64 = 1.8 as i64;
    const KELVIN_INT: i64 = 273;
    const CELSISUS_INT: i64 = 32;

    pub struct Celsius;

    pub trait ToFarenheit<T> {
        fn to_fahrenheit(&self, temp_in: T) -> (T, units::Units);
    }

    impl ToFarenheit<f64> for Celsius {
        fn to_fahrenheit(&self, temp_in: f64) -> (f64, units::Units) {
            return ((CELSISUS_CONV_FLOAT * temp_in) + CELSISUS_FLOAT, units::Units::F);
        }
    }

    impl ToFarenheit<i64> for Celsius {
        fn to_fahrenheit(&self, temp_in: i64) -> (i64, units::Units) {
            return ((CELSISUS_CONV_INT * temp_in) + CELSISUS_INT, units::Units::F);
        }
    }

    pub trait ToKelvin<T> {
        fn to_kelvin(&self, temp_in: T) -> (T, units::Units);
    }

    impl ToKelvin<f64> for Celsius {
        fn to_kelvin(&self, temp_in: f64) -> (f64, units::Units) {
            return (temp_in + KELVIN_FLOAT, units::Units::K);
        }
    }
    
    impl ToKelvin<i64> for Celsius {
        fn to_kelvin(&self, temp_in: i64) -> (i64, units::Units) {
            return (temp_in + KELVIN_INT, units::Units::K);
        }
    }
}