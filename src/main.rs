mod row_arithmetic;
use rust_backend::run;

fn main()
{
    match run()
    {
        Ok(result) => println!("{}", result),
        Err(result) => println!("{}", result)
    };
}