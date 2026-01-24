use std::thread::sleep;
use std::time::Duration;

const H_MARG: f64 = 15.0;
const V_MARG: f64 = 15.0;

const H_WIDTH: f64 = 4.0;
const V_WIDTH: f64 = 4.0;

const H_PERI: f64 = 150.0;
const V_PERI: f64 = 150.0;

const CAR_WIDTH: f64 = 2.0;
const CAR_LENGHT: f64 = 4.0;

const MAX_SPEED: f64 = 200.0 * (1000.0/3600.0); //km to m/s
const MAX_ACCEL: f64 = 3.0;
const MIN_ACCEL: f64 = -10.0;

//2 cars simulation (within the perimeter)
//returning if there was colision or not
fn cars_sim(car_via1: char, car_accel1: f64, car_via2: char, car_accel2: f64) -> bool {
    //car 1 atributes
    let chassis1 = 0001;
    let via1 = car_via1;
    let max_accel1 = MAX_ACCEL;
    let min_accel1 = MIN_ACCEL;
    let max_speed1 = MAX_SPEED;
    let lenght1 = CAR_LENGHT;
    let mut current_pos1 = -80.0;
    let mut current_speed1 = 0.0;
    let current_accel1: f64;

    //car 2 atributes
    let chassis2 = 0002;
    let via2 = car_via2;
    let max_accel2 = MAX_ACCEL;
    let min_accel2 = MIN_ACCEL;
    let max_speed2 = MAX_SPEED;
    let lenght2 = CAR_LENGHT;
    let mut current_pos2 = -100.0;
    let mut current_speed2 = 0.0;
    let current_accel2: f64;

    current_accel1 = car_accel1;
    current_accel2 = car_accel2;

    println!(" = Simulation start = ");
    let mut tickms: f64;

    loop {
        sleep(Duration::from_millis(100));
        tickms = 100.0;

        //Car 1 update
        let old_pos = current_pos1;
        current_pos1 = current_pos1 + current_speed1 * (tickms/1000.0) + current_accel1 * (tickms/1000.0) * (tickms/1000.0) / 2.0;
        current_speed1 = current_speed1 + current_accel1 * (tickms/1000.0);

        //Car 1 restrictions
        if current_pos1 < old_pos {
            current_pos1 = old_pos;
        }

        if current_speed1 < 0.0 {
            current_speed1 = 0.0;
        }

        if current_speed1 > max_speed1 {
            current_speed1 = max_speed1;
        }

        println!("Car 1 {} in {}{} position, reaching {} speed with {} acceleration.", chassis1, via1, current_pos1, current_speed1, current_accel1);

        //Car 2 update
        let old_pos = current_pos2;
        current_pos2 = current_pos2 + current_speed2 * (tickms/1000.0) + current_accel2 * (tickms/1000.0) * (tickms/1000.0) / 2.0;
        current_speed2 = current_speed2 + current_accel2 * (tickms/1000.0);

        //Car 2 restrictions
        if current_pos2 < old_pos {
            current_pos2 = old_pos;
        }

        if current_speed2 < 0.0 {
            current_speed2 = 0.0;
        }

        if current_speed2 > max_speed2 {
            current_speed2 = max_speed2;
        }

        println!("Car 2 ({}) in {}{} position, reaching {} speed with {} acceleration.", chassis2, via2, current_pos2, current_speed2, current_accel2);
    }

}

fn main(){
    //
}
