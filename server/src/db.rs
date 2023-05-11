/*
pub fn get_user(login: String, password_hash: String) -> User {
    if user_exist(login) {
        connect_user(login, password_hash);
    }
    else {
       create_user(login, password_hash);
    }
}

fn user_exist(login: String) -> bool {
    db::select_user(login);

}
*/

use csv;
use serde::{Deserialize, Serialize};
#[warn(unused_imports)]
use std::{error::Error, io, process};

#[derive(Debug,  Deserialize, Serialize)]
pub struct User{
    login: String,
    hashpass: String,
}
/*
pub fn select_users() -> Result<(), Box<dyn Error>> {

    let mut reader = csv::Reader::from_path("users.csv")?;

    for result in reader.deserialize::<User>() {
        let user: User = result?;
        println!("{:?}", user);
    }
    Ok(())
}
*/
pub enum UserCheckRes<User> {
    Verified(User),
    WrongPassword,
}

pub fn get_user(login: String, hashpass: String) -> Result<UserCheckRes<User>, Box<dyn Error>> {


    let mut reader = csv::Reader::from_path("users.csv")?;

    for result in reader.deserialize::<User>() {
        let user: User = result?;
        if user.login == login {
            if user.hashpass == hashpass {
                return Ok(UserCheckRes::Verified(user));
            }
            else {
                return Ok(UserCheckRes::WrongPassword);
            }
        }
    }
//    user = create_user

    println!("User not found. Creating...");
    create_user(login, hashpass)
}

fn create_user(login: String, hashpass: String) -> Result<UserCheckRes<User>, Box<dyn Error>> {
    let user = User{login, hashpass};

    let mut wrt = csv::WriterBuilder::new().from_path("users.csv")?;
   //wrt.write_record(&[&user.login, &user.hashpass])?;
   //wrt.write_record(&["login", "hashpass"])?;
    wrt.serialize(&user)?;
//    wrt.flush()?;
    println!("user was succesfully created!!");
   Ok(UserCheckRes::Verified(user))

}


/*
fn main() {
    if let Err(err) = select_users() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
*/







/*
 *
 * select_user (concrete)
 * delete user
 * *edit user
 *
 * make a rent
 * get all rents
 * check availability
 * delete rent
 * *edit rent
 *
 *
 *
 *
 *
 *
 *
 *
 */


