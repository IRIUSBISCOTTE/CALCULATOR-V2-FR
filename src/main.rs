use std::io::{self, Write};
use std::{thread, time};
use colored::*;

fn main() {
    loop {
        let console_chose = "[Nombre a multiplier]>>> ";
        let console_mult = "[Par combien ?]>>> ";

        print!("{}", console_chose.to_string().green().bold());
        std::io::stdout().flush().unwrap();

        let mut num_chose = String::new();
        let mut num_mult = String::new();
        let mut arreter = String::new();

        io::stdin().read_line(&mut num_chose).unwrap();

        print!("{}", console_mult.to_string().green().bold());
        std::io::stdout().flush().unwrap();

        io::stdin().read_line(&mut num_mult).unwrap();

        let n: f64 = num_chose.trim().parse().unwrap();
        let b: f64 = num_mult.trim().parse().unwrap();
        number(n, b);


        let stop_prog = "[Arreter o/n ?]>>> ";
        print!("{}", stop_prog.green().bold());
        std::io::stdout().flush().unwrap();
        io::stdin().read_line(&mut arreter).unwrap();

        if arreter.trim() == "o" {
            let prog_go_sleep = "[Programme vas s'étendre !]";
            print!("{}\n", prog_go_sleep.green().bold());
            thread::sleep(time::Duration::from_millis(1000));
            break;
        }

        fn number(n: f64, b: f64) {
            let result = "[Résultat]>>> ";
            let a = n * b;
            println!("{}  {}", result.green().bold(), a.to_string().blue().bold());
            thread::sleep(time::Duration::from_millis(500));
        }
    }//end loop
}// end main