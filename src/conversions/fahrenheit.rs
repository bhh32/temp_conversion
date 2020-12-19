pub mod from_farenheight {
    const CELSISUS_CONV_FLOAT: f64 = 1.8;
    const KELVIN_FLOAT: f64 = 273.0;
    const CELSISUS_CONV_INT: i64 = 1.8 as i64;
    const KELVIN_INT: i64 = 273;
    const CELSISUS_FLOAT: f64 = 32.0;
    const CELSISUS_INT: i64 = 32;

    pub struct Farenheit;

    pub trait ToCelsisus<T> {
        fn to_celsisus(&self, temp_in: T) -> T;
    }

    impl ToCelsisus<f32> for Farenheit {
        fn to_celsisus(&self, temp_in: f32) -> f32 {
            return (temp_in - CELSISUS_FLOAT as f32) / CELSISUS_CONV_FLOAT as f32;
        }
    }

    impl ToCelsisus<i32> for Farenheit {
        fn to_celsisus(&self, temp_in: i32) -> i32 {
            return (temp_in - CELSISUS_INT as i32) / CELSISUS_CONV_INT as i32;
        }
    }

    impl ToCelsisus<f64> for Farenheit {
        fn to_celsisus(&self, temp_in: f64) -> f64 {
            return (temp_in - CELSISUS_FLOAT) / CELSISUS_CONV_FLOAT;
        }
    }

    impl ToCelsisus<i64> for Farenheit {
        fn to_celsisus(&self, temp_in: i64) -> i64 {
            return (temp_in - CELSISUS_INT) / CELSISUS_CONV_INT;
        }
    }

    pub trait ToKelvin<T> {
        fn to_kelvin(&self, temp_in: T) -> T;
    }

    impl ToKelvin<f32> for Farenheit {
        fn to_kelvin(&self, temp_in: f32) -> f32 {
            return Farenheit.to_celsisus(temp_in) + KELVIN_FLOAT as f32;
        }
    }

    impl ToKelvin<i32> for Farenheit {
        fn to_kelvin(&self, temp_in: i32) -> i32 {
            return Farenheit.to_celsisus(temp_in) + KELVIN_INT as i32;
        }
    }

    impl ToKelvin<f64> for Farenheit {
        fn to_kelvin(&self, temp_in: f64) -> f64 {
            return Farenheit.to_celsisus(temp_in) + KELVIN_FLOAT;
        }
    }
    
    impl ToKelvin<i64> for Farenheit {
        fn to_kelvin(&self, temp_in: i64) -> i64 {
            return Farenheit.to_celsisus(temp_in) + KELVIN_INT;
        }
    }
}