use rocket::Rocket;

use std::sync::Mutex;
use std::collections::HashMap;

use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use serde::{Serialize, Deserialize};
use std::convert::TryInto;



#[derive(Serialize, Deserialize, Debug)]
struct Measure {
    id: Option<u32>,
    session: u8,
    setup: u8,
    sensor: u8,
    data: u32
}

type MeasureMap = Mutex<HashMap< u32, Measure>>; // Store on a cashed Hashmap


pub fn rocket() -> Rocket {
    rocket::ignite()
    .mount("/rpm/", routes![index])
    .mount("/api/", routes![ postmeasure, getmeasure])
    .manage(Mutex::new(HashMap::<u32, Measure>::new()))
}

#[get("/")]
fn index() -> &'static str {
    "INDEX PAGE FOR THE RPM SENSOR API"
}

#[post("/add", format = "json", data = "<measure>")]
fn postmeasure( measure: Json<Measure>, map: State<MeasureMap>) -> JsonValue {
    let mut hashmap = map.lock().expect("map lock.");
    let id: u32 = hashmap.len().try_into().unwrap();
    hashmap.insert(id , measure.0);
    json!({ "status": "ok","id": id }) 
}


#[get("/measure/<id>", format = "json")]
fn getmeasure(id : u32, map: State<MeasureMap>) ->  JsonValue {
   let hashmap = map.lock().unwrap();
 
    json!(hashmap.get(&id))
}



