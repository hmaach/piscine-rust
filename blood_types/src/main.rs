use blood_types::*;

fn main() {
	let blood_type: BloodType = "O+".parse().unwrap();
	println!("recipients of O+ {:?}", blood_type.recipients());
	println!("donors of O+ {:?}", blood_type.donors());
	let another_blood_type: BloodType = "A-".parse().unwrap();
	println!(
		"donors of O+ can receive from {:?} {:?}",
		&another_blood_type,
		blood_type.can_receive_from(&another_blood_type)
	);
}