use std::io;

fn main() {
	temperature_controller()
}


fn temperature_controller(){
	let mut temp_type = String::new();

	println!("Are you converting from C or from F?");
	io::stdin().read_line(&mut temp_type)
    .expect("Failed to read line");


	let mut temp_start = String::new();

	println!("What temperature are you converting to?");
	io::stdin().read_line(&mut temp_start)
    .expect("Failed to read line");




    let temp_start_int: f64 = temp_start.trim().parse::<f64>().unwrap();

    println!("{:?}",  temp_start.trim().parse::<f64>().unwrap());

    if temp_type.trim() == "C"{
	    print_temp("F", temp_start_int, convert_to_f(temp_start_int));

    }else{
	    print_temp("C", temp_start_int, convert_to_c(temp_start_int));

    }
}


fn convert_to_f(c:f64) -> f64{
	c*(1.8) + 32.0
}


fn convert_to_c(f:f64) -> f64{
 ((f - 32.0) *5.0)/9.0
}

fn print_temp(final_type: &str, org_temp: f64, final_temp: f64){
	let types = if final_type == "C" {
		("F", "C")
	}else{
		("C", "F")
	};
	println!("++++++");
	println!("Temp started as {} {}", org_temp ,types.0);
	println!("Temp ended as {} {}", final_temp ,types.1);

}