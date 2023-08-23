#[macro_use] extern crate rocket;

mod row_arithmetic;
use rust_backend::run;
use rocket::serde::json::Json;

#[get("/")]
fn hello_world() -> &'static str
{
    "Hello, world!"
}


#[post("/", data = "<linear_program_json>")]
fn index(linear_program_json: Json<row_arithmetic::LinearProgram>) -> Json<String>
{
    let linear_program_json_string: String = match linear_program_json.to_json()
    {
        Ok(json_string) => json_string,
        Err(error) => return Json(error)
    };

    match run(&linear_program_json_string)
    {
        Ok(json_response) => Json(json_response),
        Err(error) => Json(error)
    }
}

#[launch]
fn rocket() -> _
{
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![hello_world])
    
}