use local_ip_address::local_ip;
use winreg::{enums::*, RegKey};

fn main() {
    let my_local_ip = local_ip().unwrap();

    let key = "LocalIp";

    let hkcu = RegKey::predef(HKEY_CURRENT_USER); // get current user
    let (env, _) = hkcu.create_subkey("Environment").unwrap(); // create_subkey opens with write permissions
    env.set_value(key, &my_local_ip.to_string()).unwrap();

    let return_env: String = env.get_value(key).unwrap();
    println!("{:?} is set with the value: {:?}", key, return_env);
    //only close window on user input
    println!("Press enter to exit");
    let mut line = String::new();
    let _input = std::io::stdin().read_line(&mut line).expect("Failed to read line");
}
