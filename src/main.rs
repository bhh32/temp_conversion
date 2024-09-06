mod conversions;

use std::io;
use conversions::celsisus::from_celsisus::*;
use conversions::fahrenheit::from_farenheight::*;
use conversions::kelvin::from_kelvin::*;

fn main() {
    let mut tempIn = 32.0;
    let f = Farenheit;
    let c = Celsius;

    println!("{}F = {}C", tempIn, f.to_celsisus(tempIn));
    
    println!("{}C = {}F", tempIn, c.to_fahrenheit(tempIn));
}