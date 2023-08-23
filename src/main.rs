#[macro_use] extern crate rocket;

mod row_arithmetic;
// use rust_backend::run;
use rocket::serde::json::Json;
use serde::Serialize;

use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

// This bit I don't really understand
pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, OPTION"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}


#[get("/")]
fn hello_world() -> Json<String>
{
    Json(String::from("Hello world"))
}

#[options("/")]
fn options() -> Json<String>
{
    Json(String::from("Options I guess"))
}

#[derive(Serialize)]
enum LinearProgramResponse
{
    LinearProgram(row_arithmetic::Row),
    Error(String)
}

#[post("/", data = "<linear_program>")]
fn index(linear_program: Json<row_arithmetic::LinearProgram>) -> Json<LinearProgramResponse>
{
    match row_arithmetic::perform_checks(&linear_program)
    {
        Ok(_) => (),
        Err(error) => return Json(LinearProgramResponse::Error(error))
    }

    let mut linear_program = linear_program.into_inner();

    linear_program.relative_costs = linear_program.calculate_costs();

    match linear_program.preform_simplex()
    {
        Ok(_) => (),
        Err(error) => return Json(LinearProgramResponse::Error(error))
    };

    match linear_program.set_solution()
    {
        Ok(_) => (),
        Err(error) => return Json(LinearProgramResponse::Error(error))
    }

    let response_row = row_arithmetic::Row{a_ij: linear_program.solution, b_i: linear_program.relative_costs.b_i};

    Json(LinearProgramResponse::LinearProgram(response_row))
}

#[launch]
fn rocket() -> _
{
    rocket::build()
        .mount("/", routes![index, hello_world, options])
        .attach(CORS)
    
}