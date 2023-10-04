#[macro_use] extern crate rocket;

pub mod row_arithmetic;
pub mod solve_linear_equations;

// use rust_backend::run;
use rocket::serde::json::Json;
use serde::Serialize;

use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

mod simplex_test;

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
pub enum LinearProgramResponse
{
    LinearProgram(row_arithmetic::Row),
    Unbound(String),
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
        row_arithmetic::SimplexResult::Finished => (),
        row_arithmetic::SimplexResult::Unbound => return Json(LinearProgramResponse::Unbound(format!("Problem is unbound and the optimal solution is infinity"))),
        row_arithmetic::SimplexResult::IterationComplete => return Json(LinearProgramResponse::Error(format!("Iteration complete, you should never get this though"))),
        row_arithmetic::SimplexResult::Error(error) => return Json(LinearProgramResponse::Error(error))
    };

    match linear_program.set_solution()
    {
        Ok(_) => (),
        Err(error) => return Json(LinearProgramResponse::Error(error))
    }

    let response_row = row_arithmetic::Row{a_ij: linear_program.solution, b_i: linear_program.relative_costs.b_i};

    Json(LinearProgramResponse::LinearProgram(response_row))
}

#[derive(Serialize)]
enum LinearEquationResult
{
    LinearEquation(solve_linear_equations::LinearSystem),
    Err(String)
}

#[post("/", data = "<linear_equation>")]
fn linear_program(linear_equation: Json<solve_linear_equations::LinearSystem>) -> Json<LinearEquationResult>
{
    let mut linear_equation = linear_equation.into_inner();

    match linear_equation.solve_system_of_linear_equations()
    {
        Ok(_) => Json(LinearEquationResult::LinearEquation(linear_equation)),
        Err(error) => Json(LinearEquationResult::Err(error)),
    }
}

#[catch(400)]
fn parsing_error(_request: &rocket::Request) -> Json<LinearProgramResponse>
{
    Json(LinearProgramResponse::Error(String::from("Failed to process JSON")))
}

#[launch]
fn rocket() -> _
{
    rocket::build()
        .mount("/", routes![index, hello_world, options])
        .mount("/linear_equation", routes![linear_program])
        .register("/", catchers![parsing_error])
        .attach(CORS)
    
}