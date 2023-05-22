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
    is_admin: bool,
}
impl User {
    pub fn get_login(self) -> String {
        return self.login
    }
}
pub enum UserCheckRes<User> {
    Verified(User),
    WrongPassword,
}
// ---- USER BLOCK
pub fn get_user(login: String, hashpass: String, with_admin_rights:bool) -> Result<UserCheckRes<User>, Box<dyn Error>> {
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
    println!("User not found. Creating...");
    create_user(login, hashpass, with_admin_rights)
}

fn create_user(login: String, hashpass: String, is_admin: bool) -> Result<UserCheckRes<User>, Box<dyn Error>> {
    let user = User{login, hashpass, is_admin};

    let mut wrt = csv::WriterBuilder::new().from_path("users.csv")?;
   //wrt.write_record(&[&user.login, &user.hashpass])?;
   //wrt.write_record(&["login", "hashpass"])?;
    wrt.serialize(&user)?;
//    wrt.flush()?;
    println!("user was succesfully created!!");
   Ok(UserCheckRes::Verified(user))
}
// ___ USER BLOCK END ___


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Booking {
    car_id: u64,
    car_model: String,
    user_login: String,
    date: String,
}
impl Booking {
    pub fn get_model(&self) -> String {
        self.clone().car_model
    }
    pub fn get_id(&self) -> u64{
        self.car_id
    }
    pub fn get_user_login(&self) -> String {
        self.clone().user_login
    }
    pub fn get_date(&self) -> String {
        self.clone().date
    }
}

#[derive(Debug,  Deserialize, Serialize)]
#[derive (Clone)]
pub struct Car{
    id: u64,
    model: String,
}
impl Car{
    pub fn get_model(&self) -> String {
        self.clone().model
    }
    pub fn get_id(&self) -> u64 {
        self.id
    }
    pub fn get_from_model_and_date(model: String, date: String) -> Result<Option<Car>, Box<dyn Error>> {
                    // getting all viecle id's for concrete car model
        let mut reader = csv::Reader::from_path("cars.csv")?;
        let mut car_ids = vec![];
        for result in reader.deserialize::<Car>() {
            let car: Car = result?;
        if car.clone().get_model() == model {
                car_ids.push(car.clone().get_id());
            }
        }

        // getting date info from bookings to check availability
        let mut reader = csv::Reader::from_path("bookings.csv")?;
        for result in reader.deserialize::<Booking>() {
            let booking: Booking = result?;

            for (i, id) in car_ids.clone().iter().enumerate() {

                if id.to_owned() == booking.get_id(){
                    if booking.date == date {
                        car_ids.remove(i);
                    }
                }
            }
        }

        let res = match car_ids.first() {

            None => Ok(None),
            Some(id) => Ok(Some(
                Car{
                    id: id.to_owned(),
                    model ,
                })),
        };
    res
    }
}

impl Booking{
   pub fn from_car(car: Car, date: String, login: String) -> Booking {
        Booking {
            car_id: car.get_id(),
            car_model: car.get_model(),
            user_login: login,
            date,

        }
    }

}

// ___ Booking BLOCK
//
/*
pub fn make_booking(car_model: String, user_login: String,
                    date: String) -> Result<(), Box<dyn Error>> {

    // getting all viecle id's for concrete car model
    let mut reader = csv::Reader::from_path("cars.csv")?;
    let mut car_ids = vec![];
    for result in reader.deserialize::<Car>() {
        let car: Car = result?;
        if car.clone().get_model() == car_model {
            car_ids.push(car.clone().get_id());
        }
    }
    // getting date info from bookings to check availability
    let mut reader = csv::Reader::from_path("bookings.csv")?;
    for result in reader.deserialize::<Booking>() {
        let booking: Booking = result?;

        for (i, id) in car_ids.iter().enumerate() {

            if id == &booking.get_id(){
                if &booking.date == &date {
                    car_ids.remove(i);
                }
            }
        }
    }

    let res = match car_ids.first() {
        None => Err("there is no way to rent this model of car in this date. All cars was already booked"),
        Some(id) => Ok(
            Booking {
                car_model,
                user_login,
                car_id: id.to_owned(),
                date,
            }),

    };



    Ok(())



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


