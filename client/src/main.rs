
use std::str;
use std::net::TcpStream;
use std::io::{self,prelude::*,BufReader,Write};


fn main() -> io::Result<( )>{
    // connect
    // Struct used to start requests to the server.
    // Check TcpStream Connection to the server
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    for _ in 0..1000 {
        // Allow sender to enter message input
        let mut input = String::new();
        // First access the input message and read it
        io::stdin().read_line(&mut input).expect("Failed to read");
        // Write the message so that the receiver can access it
        stream.write(input.as_bytes()).expect("failed to write");
        // Add buffering so that the receiver can read messages from the stream
        let mut reader =BufReader::new(&stream);
        // Check if this input message values are u8
        let mut buffer: Vec<u8> = Vec::new();
        // Read input information
        reader.read_until(b'\n',&mut buffer)?;

        println!("read from server:{}",str::from_utf8(&buffer).unwrap());
        println!("");
        signin();
    }
    Ok(())
}


fn check_pass(pass: String) -> bool {

    //. . . . . .. .
    //......
    return pass == pass;

}

struct User {
    id: u64,
    login: String,
    pass_hash: String,
}
use server::create_user;
fn signin()  -> User { println!("Process of signing in started\nEnter your login what already exist or which you wonna create"); // entering login loop {
                                                                                                                        // 00
    loop {                                                                                                                        //
                                                                                                                        //
        let mut login = &mut String::new();
        let stdin = io::stdin();

        stdin.read_line(login).unwrap();
        login.pop();

        if login.chars().all(|c| c.is_alphabetic()) {
        }
        else {
                println!("Use only latin alphabet symbols\nTry again");
                println!("Your input: {}", login);
                continue;
        }

        println!("Ok, now enter password for login {login}. If login doesn't exits, it'll be created with this password");
        let mut pass = &mut String::new();
        let stdin = io::stdin();
        stdin.read_line(pass).unwrap();
        if check_pass(pass.clone()) == false {
            println!("Access denied. Password didn't match");
            continue;
        }
        println!("Assess granted");

        user = db::get_user(login, password);


    }

}





