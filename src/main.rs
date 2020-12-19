mod conversions;

use std::io;
use conversions::celsisus::from_celsisus::*;
use conversions::fahrenheit::from_farenheight::*;
use conversions::usr_input::user_input::*;

fn main() {
    /*
    let mut temp_in = 32.0;
    let f = Farenheit;
    let c = Celsius;

    println!("{}F = {}C", temp_in, f.to_celsisus(temp_in));
    let temp_in = c.to_fahrenheit(temp_in);
    println!("{}{} = {}{}", 32.0, units::Units::C, temp_in.0, temp_in.1);
    */
    let mut temp_in = 32.0;
    let f = Farenheit.to_celsisus(temp_in);
    let c = Celsius;
    //f.to_celsisus(temp_in);
    let input = Input;
    //input.get_input();
}