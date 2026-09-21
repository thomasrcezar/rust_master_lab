// understading macros
macro_rules! say_hello{
    () => {
        println!("Hello, rust!");
    }
}
macro_rules! info{
    ($message:expr) => {
        println!("[INFO]: {}", $message);
    }
}

macro_rules! check_status{
    ($condition:expr)=>{
        if $condition{
            println!("OK")
        }
        else{
            println!("Failed")
        }
    }
}


fn main() {
    // A normal variable stored in the progam. 
    let ip_address = "127.0.0.1";
    println!("IP address: {}", ip_address);

    // A macro is a way to generate code at compile time.
    let message = format!("IP address: {}", ip_address);
    println!("Message: {}", message);


    let devices = vec![
        "192.168.0.160",
        "192.168.0.161",
        "192.168.0.162",
        "192.168.0.163",
    ];
    for ip in &devices {
        println!("Device: {}", ip);
    }
    println!("Devices: {:?}", devices);
    say_hello!();
    info!("Devices found");
    check_status!(devices.len() > 0);
}
