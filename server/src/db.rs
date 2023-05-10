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
use serde::Deserialize;
use std::{error::Error, io, process};

#[derive(Debug, serde::Deserialize)]
struct User{

    login: String,
//     #[serde(deserialize_with = "csv::invalid_option")]
    id: u32,   password_hash: String,
}

pub fn select_users() -> Result<(), Box<dyn Error>> {
     let csv = "login,password_hash,id
        1948,Porsche,356
        1967,Ford,228";

    let mut reader = csv::Reader::from_path("users.csv")?;

    for result in reader.deserialize::<User>() {
        let user: User = result?;
        println!("{:?}", user);
    }
/*
    for record in reader.records() {
        let record = record?;
        println!(
            "In {}, {} built the {} model. It is a {}.",
            &record[0],
            &record[1],
            &record[2],
            &record[3]
        );
    }
    */
    /*
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        println!("{:?}", result);
        let user: User= result?;
        println!("{:?}", user);
    }
    */
    Ok(())
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


