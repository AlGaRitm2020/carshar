use std::time;
use std ::net::{TcpListener,TcpStream};
use std::io::{Read,Write};
use std::thread;

// Handle access stream
//create a struct to hold the stream’s state
// Perform I/O operations
fn handle_sender(mut stream: TcpStream) -> io::Result<()>{
    // Handle multiple access stream
    let mut buf = [0;512];
    for _ in 0..1000{
        // let the receiver get a message from a sender
        let bytes_read = stream.read(&mut buf)?;
        // sender stream in a mutable variable
        if bytes_read == 0{
            return Ok(());
        }
        stream.write(&buf[..bytes_read])?;
        // Print acceptance message
        //read, print the message sent
        println!("from the sender:{}",String::from_utf8_lossy(&buf));
        // And you can sleep this connection with the connected sender
        thread::sleep(time::Duration::from_secs(1));
    }
    // success value
    Ok(())
}
use sha256::{digest};
use std::{error::Error, io, process};
mod db;
use db::get_user;
use db::User;
fn main() -> io::Result<()>{


    loop {
        let stdin = io::stdin();
        let mut login =&mut String::new();
        let passhash =&mut String::new();
        stdin.read_line(login).unwrap();
        login.pop();

        stdin.read_line(passhash).unwrap();
        passhash.pop();

        let passhash = digest(passhash.clone());

/*
        if let Err(err) = get_user(login.to_string(), passhash.to_string(), true) {
            println!("error running example: {}", err);
            process::exit(1);
        }
*/
        match get_user(login.to_string(), passhash.to_string(), true) {
            Err(err) => {println!("error running example: {}", err);
                        process::exit(1);},
            Ok(db::UserCheckRes::WrongPassword) => {
                println!("Wrong password\nTry again..");

            },

            Ok(db::UserCheckRes::Verified(user)) => {
                let user: User = user;
                println!("Access granted for {}", &user.get_login());
                break;

            }

        }
    }
    let receiver_listener = TcpListener::bind("127.0.0.1:7878").expect("Failed and bind with the sender");
    // Getting a handle of the underlying thread.
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
    // listen to incoming connections messages and bind them to a sloginever socket address.
    for stream in receiver_listener.incoming() {
        let stream = stream.expect("failed");
        // let the receiver connect with the sender
        let handle = thread::spawn(move || {
            //receiver failed to read from the stream
            handle_sender(stream).unwrap_or_else(|error| eprintln!("{:?}",error))
        });

        // Push messages in the order they are sent
        thread_vec.push(handle);
    }

    for handle in thread_vec {
        // return each single value Output contained in the heap
        handle.join().unwrap();
    }
    // success value
    Ok(())
}

