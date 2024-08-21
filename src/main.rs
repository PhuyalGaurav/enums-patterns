fn main () {
    let gaurav_phone: Option<u64> = Some(9766915125u64);

    // This match code can be simplied into if let.
    match gaurav_phone {
        Some(phone) => {
            println!("Gaurav's number is {phone}");
        },
        None => {
            println!("Gaurav's number is not there")
        }
    }

    // if we didnt want to use all of the other possible cases it saves us from builer plate _ => () code.
    if let Some(phone) = gaurav_phone {
        println!("gaurav's number is {phone}");
    }
    else {
        println!("Gaurav's number is not given");
    }
}