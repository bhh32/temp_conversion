pub mod from_farenheight {
    const CELSISUS_CONV_FLOAT: f64 = 1.8;
    const kelvin_float: f64 = 273.0;
    const celsisus_conv_int: i64 = 1.8 as i64;
    const kelvin_int: i64 = 273;
    const celsisus_float: f64 = 32.0;
    const celsisus_int: i64 = 32;

    pub struct Farenheit;

    pub trait ToCelsisus<T> {
        fn to_celsisus(&self, T: T) -> T;
    }

    impl ToCelsisus<f32> for Farenheit {
        fn to_celsisus(&self, temp_in: f32) -> f32 {
            return (temp_in - celsisus_float as f32) / CELSISUS_CONV_FLOAT as f32;
        }
    }

    impl ToCelsisus<i32> for Farenheit {
        fn to_celsisus(&self, temp_in: i32) -> i32 {
            return (temp_in - celsisus_int as i32) / celsisus_conv_int as i32;
        }
    }

    impl ToCelsisus<f64> for Farenheit {
        fn to_celsisus(&self, temp_in: f64) -> f64 {
            return (temp_in - celsisus_float) / CELSISUS_CONV_FLOAT;
        }
    }

    impl ToCelsisus<i64> for Farenheit {
        fn to_celsisus(&self, temp_in: i64) -> i64 {
            return (temp_in - celsisus_int) / celsisus_conv_int;
        }
    }

    pub trait ToKelvin<T> {
        fn to_kelvin(&self, T: T) -> T;
    }

    impl ToKelvin<f32> for Farenheit {
        fn to_kelvin(&self, temp_in: f32) -> f32 {
            return Farenheit.to_celsisus(temp_in) + kelvin_float as f32;
        }
    }

    impl ToKelvin<i32> for Farenheit {
        fn to_kelvin(&self, temp_in: i32) -> i32 {
            return Farenheit.to_celsisus(temp_in) + kelvin_int as i32;
        }
    }

    impl ToKelvin<f64> for Farenheit {
        fn to_kelvin(&self, temp_in: f64) -> f64 {
            return Farenheit.to_celsisus(temp_in) + kelvin_float;
        }
    }
    
    impl ToKelvin<i64> for Farenheit {
        fn to_kelvin(&self, temp_in: i64) -> i64 {
            return Farenheit.to_celsisus(temp_in) + kelvin_int;
        }
    }
}