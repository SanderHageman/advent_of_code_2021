use colored::Colorize;
use std::{fs, io::Result, time};

#[macro_export]
macro_rules! main {
    ($(day $val:expr)+) => {
        paste!{
            $(mod [<day $val>];)+
            pub fn main() {
                let args = std::env::args()
                    .skip(1)
                    .map(|x| x.parse::<u8>().unwrap())
                    .collect::<Vec<_>>();

                main_util::print_entry(&args);

                let duration = main_util::time_duration(||{
                    $(if args.is_empty() || args.contains(&$val){
                            main_util::do_day($val, [<day $val>]::day);
                    })+
                });
                println!("Execution took {}ms", duration.as_millis());
            }
        }
    };
}

pub fn do_day<F, T, T2>(day: usize, fn_day: F)
where
    F: Fn(String) -> (T, T2),
    T: std::fmt::Display,
    T2: std::fmt::Display,
{
    let mut result = None;
    let input = get_input(day, 2021);
    let duration = time_duration(|| {
        result = Some(fn_day(input));
    });

    if let Some(result) = result {
        print_result(day, result, duration.as_millis());
    }
}

pub fn time_duration<F>(function: F) -> time::Duration
where
    F: FnOnce(),
{
    let start = time::Instant::now();
    function();
    time::Instant::now().duration_since(start)
}

pub fn print_entry(filter: &Vec<u8>) {
    println!(
        // Thanks Caspar
        "\t{} {} {} {} {:?}",
        "Advent".bright_red().bold(),
        "of".bright_white(),
        "Code".bright_green().bold(),
        "2021".bright_blue(),
        filter
    );
}

pub fn print_result<T, T2>(day: usize, result: (T, T2), time: u128)
where
    T: std::fmt::Display,
    T2: std::fmt::Display,
{
    let resd = "Result ";
    let day = format!("day {:02}", day);
    let time = format!("{:05}ms", time);
    let res1 = format!("{:<14}", result.0);
    let res2 = format!("{:<10}", result.1);
    println!(
        "{}{} ({}): {} | {}",
        resd.green(),
        day.bright_blue(),
        time.dimmed(),
        res1,
        res2,
    );
}

pub fn get_input(day: usize, year: usize) -> String {
    let file_path = format!("input/day{:02}", day);
    let file_content = fs::read_to_string(&file_path);

    {
        file_content.unwrap_or_else(|_| {
            println!("Fetching input for {}/{} online", day, year);
            let result = get_online_input(day, year).expect("Unable to fetch input");
            fs::write(&file_path, &result).expect("Unable to write to cache");
            result
        })
    }
    .trim()
    .to_owned()
}

fn get_online_input(day: usize, year: usize) -> Result<String> {
    let session_id = fs::read_to_string("input/session_id")
        .expect("Unable to read session id at input/session_id")
        .trim()
        .to_owned();

    let response = ureq::get(&format!(
        "https://adventofcode.com/{}/day/{}/input",
        year, day
    ))
    .set("Cookie", &format!("session={}", session_id))
    .call();

    let resp = response.into_string()?;
    if resp.starts_with(TOO_EARLY) {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Day {} {} has not started yet", day, year),
        ))
    } else {
        Ok(resp)
    }
}

const TOO_EARLY: &str = "Please don't repeatedly request this endpoint before it unlocks!";
