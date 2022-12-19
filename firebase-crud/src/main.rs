use dotenvy::dotenv;
use firebase_rs::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u32,
    email: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Response {
    name: String,
}
#[tokio::main]
async fn main() {
    dotenv().expect(".env File not Found");
    let db_uri = env::var("DB_URI").unwrap();
    let user = User {
        name: "Kundan Karna".to_string(),
        age: 28,
        email: "kundan.karna1994@gmail.com".to_string(),
    };

    let firebase = Firebase::new(&db_uri).unwrap();
    let response = set_user(&firebase, &user).await;
    println!("Set User : {:?}", response);

    let mut user = get_user(&firebase, &response.name).await;
    println!("Get User : {:?}", user);

    let users = get_users(&firebase).await;
    println!("GET USERS {:?}", users);

    user.email = "kundan.karna@gmail.com".to_string();

    let updated_user = update_user(&firebase, &response.name, &user).await;

    println!("Updated User {:?}", updated_user);

    delete_user(&firebase, &response.name).await;
}

async fn set_user(firebase_client: &Firebase, user: &User) -> Response {
    let firebase = firebase_client.at("users");
    let _user = firebase.set(user).await;

    return string_to_response(&_user.unwrap().data);
}

async fn get_user(firebase_client: &Firebase, id: &String) -> User {
    let firebase = firebase_client.at("users").at(&id);
    let _user = firebase.get::<User>().await;

    return _user.unwrap();
}

async fn get_users(firebase_client: &Firebase) -> HashMap<String, User> {
    let firebase = firebase_client.at("users");
    let users = firebase.get::<HashMap<String, User>>().await;
    return users.unwrap();
}

async fn update_user(firebase_client: &Firebase, id: &String, user: &User) -> User {
    let firebase = firebase_client.at("users").at(&id);
    let _user = firebase.update::<User>(&user).await;
    return string_to_user(&_user.unwrap().data);
}

async fn delete_user(firebase_client: &Firebase, id: &String) {
    let firebase = firebase_client.at("users").at(&id);
    let _deleted = firebase.delete().await;
    println!("{} deleted Successfully", id);
}

//convert a string to a response
fn string_to_response(s: &str) -> Response {
    serde_json::from_str(s).unwrap()
}

//convert a string to user
fn string_to_user(s: &str) -> User {
    serde_json::from_str(s).unwrap()
}
