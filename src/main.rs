extern crate hyper;
use hyper::Client;

extern crate clap;
use clap::{Arg, App, SubCommand, ArgMatches};

use std::str;

const A_ENDPOINT: &'static str = "endpoint";
const A_THREADS: &'static str = "threads";
const A_REQUESTS: &'static str = "requests";
const A_BODY: &'static str = "body";
const A_HEADERS: &'static str = "headers";

#[derive(Debug)]
enum TaskError {
    ParseError,
}

impl From<std::num::ParseIntError> for TaskError {
    fn from(err: std::num::ParseIntError) -> TaskError {
        return TaskError::ParseError;
    }
}

#[derive(Debug)]
struct Header {
    key: String,
    values: Vec<String>,
}

#[derive(Debug)]
struct Task<'a> {
    endpoint: &'a str,
    body: Option<&'a str>,
    headers: Vec<Header>,
    threads: u32,
    requests: u32,
}

impl str::FromStr for Header {
    type Err = TaskError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split_res: Vec<&str> = s.split(':').collect();
        if split_res.len() > 0 {
            let key: String = split_res[0].to_string();
            let mut vals: Vec<String> = Vec::new();
            for i in 1..split_res.len() {
                vals.push(split_res[i].to_string());
            }
            return Ok(Header {
                key: key,
                values: vals,
            });
        } else {
            return Err(TaskError::ParseError);
        }
    }
}


impl<'a> Task<'a> {
    pub fn new_from_input(inputs: &'a ArgMatches) -> Result<Task<'a>, TaskError> {
        let mut result = Task {
            endpoint: inputs.value_of(A_ENDPOINT).unwrap(),
            body: inputs.value_of(A_BODY),
            headers: Vec::new(),
            threads: 0,
            requests: 0,
        };

        let thread_str: String = inputs.value_of(A_THREADS).unwrap().to_string();
        result.threads = try!(thread_str.parse::<u32>());

        let req_str: String = inputs.value_of(A_REQUESTS).unwrap().to_string();
        result.requests = try!(req_str.parse::<u32>());

        for header in inputs.values_of(A_HEADERS).unwrap() {
            result.headers.push(try!(header.parse::<Header>()));
        }
        return Result::Ok(result);
    }
}

fn main() {
    let inputs: ArgMatches = App::new("dumbbell")
        .version("1.0")
        .about("A Load Tester")
        .author("Ryan Bluth")
        .arg(Arg::with_name(A_ENDPOINT)
            .help("The endpoint to target")
            .short("e")
            .takes_value(true))
        .arg(Arg::with_name(A_THREADS)
            .help("The number of threads to use for sending requests")
            .default_value("10")
            .short("t")
            .takes_value(true))
        .arg(Arg::with_name(A_REQUESTS)
            .help("The number of requests to send. 0 == infinite")
            .short("r")
            .default_value("100")
            .takes_value(true))
        .arg(Arg::with_name(A_BODY)
            .help("The body of the HTTP Request")
            .short("b")
            .default_value("Hello World")
            .takes_value(true))
        .arg(Arg::with_name(A_HEADERS)
            .multiple(true)
            .help("The HTTP headers in format header-key:value1:value2")
            .short("h")
            .takes_value(true))
        .get_matches();

    let task: Task = Task::new_from_input(&inputs).unwrap();
    println!("{:?}", task);
}
