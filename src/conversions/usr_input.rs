pub mod user_input {
    use crate::units;
    use std::io;

    pub struct Input;

    trait GetInput<T> {
        fn get_input() -> (T, units::Units);
    }

    impl GetInput<i64> for Input {
        fn get_input() -> (i64, units::Units) {
            let mut temp = String::new();
            println!("Enter the temperature you'd like to convert: ");
        
            io::stdin().read_line(& mut temp).expect("");
            let temp: i64 = temp.trim().parse().unwrap();

            println!("Please enter the unit of measurment: ");

            let mut choice = String::new();
            for i in 1..3 {
                if i == 1 {
                    println!("1.) F");
                } else if i == 2 { 
                    println!("2.) C");
                } else {
                    println!("3.) K");
                }
            }

            io::stdin().read_line(&mut choice).expect("");
            let choice: i64 = choice.trim().parse().unwrap();

            let mut usr = (0, units::Units::F);
            if choice == 1 {
                usr = (temp, units::Units::F);
            } else if choice == 2 {
                usr = (temp, units::Units::C);
            } else if choice == 3 {
                usr = (temp, units::Units::K);
            } else {
                usr = (temp, units::Units::Unknown);
            }

            return usr;
        }
    }
}