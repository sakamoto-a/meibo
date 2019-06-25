use std::io;
use std::vec::Vec;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::{BufWriter, Write};

const PROFILE_VEC_SIZE: usize = 10000;

struct Date{
    year: u16,
    month: u16,
    day: u16,
}

struct Profile{
    id: u32,
    name: String,
    date: Date,
    place: String,
    data: String,
}

fn print_profile(data_num: usize, profile_data: &Vec<Profile>)
{
    println!("id: {}", profile_data[data_num].id);
    println!("name: {}", profile_data[data_num].name);
    println!("date: {}-{}-{}", profile_data[data_num].date.year, profile_data[data_num].date.month, profile_data[data_num].date.day);
    println!("place: {}", profile_data[data_num].place);
    println!("data: {}", profile_data[data_num].data);
    println!("");
}

fn join_profile(data_num: usize, profile_data: &Vec<Profile>) -> String
{
    profile_data[data_num].id.to_string() + "," + &profile_data[data_num].name + "," + &profile_data[data_num].date.year.to_string() + "-" + &profile_data[data_num].date.month.to_string() + "-" + &profile_data[data_num].date.day.to_string() + "," + &profile_data[data_num].place + "," + &profile_data[data_num].data + "\n"
}

fn exec_quit()
{
    std::process::exit(0);
}

fn exec_check(profile_data: &mut Vec<Profile>)
{
    println!("{} profile(s)", profile_data.len());
}

fn exec_print(param: &str, profile_data: &mut Vec<Profile>)
{
    let nitems:i32;
    let arg_check = param.parse();
    match arg_check {
        Ok(arg) => nitems = arg,
        Err(_) => {
            println!("incorrect value for print");
            return;
        },
    };

    if nitems > 0 {
        let mnitems = nitems as usize;
        if mnitems > profile_data.len() {
            println!("too big arg");
        } else {
            for i in 0..mnitems {
                print_profile(i, &profile_data);
            }
        }
    } else if nitems == 0 {
        for i in 0..profile_data.len() {
            print_profile(i, &profile_data);
        }
    } else {
        let mnitems = nitems.abs() as usize;
        if mnitems > profile_data.len() {
            println!("too small arg");
        } else {
            for i in (profile_data.len() - mnitems).. profile_data.len() {
            print_profile(i, &profile_data);
            }
        }
    }
}

fn exec_write(param: &str, profile_data: &mut Vec<Profile>)
{
    let file_exist = File::open(param);
    match file_exist {
        Ok(_) =>  {
            println!("{} is exist. overload? y/n", param);
            let mut line;

            loop {
                line = get_line();
                line = line.replace("\n", "");
                if line == "y".to_string() {
                    break;
                } else if line == "n".to_string(){
                    return;
                } else {
                    println!("input y/n");
                }
            }
        },
        Err(_) => (),
    }
    let read_file = File::create(param);
    match read_file {
        Ok(n) => {
            let mut buf = BufWriter::new(n);
            for i in 0 .. profile_data.len() {
                buf.write(join_profile(i, profile_data).as_bytes()).unwrap();
            }
        },
        Err(_) => println!("{} can not create", param),
    }
}

fn exec_read(param: &str, profile_data: &mut Vec<Profile>)
{
    let read_file = File::open(param);
    match read_file {
        Ok(n) => for line in BufReader::new(n).lines() {
            new_profile(line.unwrap(), profile_data);
        },
        Err(_) => println!("{} not found", param),
    }
}

fn exec_sort(param: &str, profile_data: &mut Vec<Profile>)
{
    let arg_num;
    let arg_check = param.parse();
    match arg_check {
        Ok(arg) => arg_num = arg,
        Err(_) => {
            println!("incorrect value for sort");
            return;
        },
    };
    match arg_num {
        1 => {
            profile_data.sort_by(|a, b| a.id.cmp(&b.id));
            println!("sort by id");
        },
        2 => {
            profile_data.sort_by(|a, b| a.name.cmp(&b.name));
            println!("sort by name");
        },
        3 => {
            profile_data.sort_by(|a, b| (a.date.year, a.date.month, a.date.day).cmp(&(b.date.year, b.date.month, b.date.day)));
            println!("sort by date");
        },
        4 => {
            profile_data.sort_by(|a, b| a.place.cmp(&b.place));
            println!("sort by place");
        },
        5 => {
            profile_data.sort_by(|a, b| a.data.cmp(&b.data));
            println!("sort by data");
        },
        _ => println!("incorrect value for sort"),
    };
}

fn exec_find(param: &str, profile_data: &mut Vec<Profile>)
{
    for data_num in 0..profile_data.len() {
        let id_string = profile_data[data_num].id.to_string();
        let year_string = profile_data[data_num].date.year.to_string();
        let month_string = profile_data[data_num].date.month.to_string();
        let day_string = profile_data[data_num].date.day.to_string();

        if id_string == param || profile_data[data_num].name == param || year_string == param || month_string == param || day_string == param || profile_data[data_num].place == param || profile_data[data_num].data == param {
            print_profile(data_num, profile_data);
        }
    }
}


fn exec_command1(cmd: &str, profile_data: &mut Vec<Profile>)
{
    match cmd {
        "%Q" => exec_quit(),
        "%C" => exec_check(profile_data),
        "%P" | "%R" | "%W" | "%S" | "%F" => println!("{} has one arg", cmd),
        _    => println!("{} is not command", cmd),
    }
}


fn exec_command2(cmd: &str, param: &str, profile_data: &mut Vec<Profile>)
{

    match cmd {
        "%P" => exec_print(param, profile_data),
        "%R" => exec_read(param, profile_data),
        "%W" => exec_write(param, profile_data),
        "%S" => exec_sort(param, profile_data),
        "%F" => exec_find(param, profile_data),
        "%C" | "%Q" => println!("{} has no arg", cmd),
        _    => println!("{} is not command", cmd),
    }
}

fn is_this_exist_date(year: u16, month: u16, day: u16) -> i8
{
    let feb;

    if year % 4 == 0 && year % 100 != 0 || year % 400 == 0{
        feb = 29;
    } else {
        feb = 28;
    }


    let month_tab: [u16; 12] = [31, feb, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    if month > 12 {
        println!("incorrect month");
        return -1;
    }

    let max_day = month_tab[(month-1) as usize];

    if day > max_day {
        println!("incorrect day");
        return -1;
    }
    0
}


fn new_profile(line: String, profile_data: &mut Vec<Profile>)
{
    let new_profile: Vec<&str> = line.splitn(5, ',').collect();
    let num_split = new_profile.len();
    let id_num: u32;
    let year_num: u16;
    let month_num: u16;
    let day_num: u16;

    if num_split == 5 {
        let new_date: Vec<&str> = new_profile[2].splitn(3, '-').collect();
        if new_date.len() == 3 {
            let id_check = new_profile[0].parse();
            match id_check {
                Ok(id) => id_num = id,
                Err(_) => {
                    println!("incorrect id");
                    return;
                },
            };

            let year_check = new_date[0].parse();
            match year_check {
                Ok(year) => year_num = year,
                Err(_) => {
                    println!("incorrect year");
                    return;
                },
            };

            let month_check = new_date[1].parse();
            match month_check {
                Ok(month) => month_num = month,
                Err(_) => {
                    println!("incorrect month");
                    return;
                },
            };

            let day_check = new_date[2].parse();
            match day_check {
                Ok(day) => day_num = day,
                Err(_) => {
                    println!("incorrect day");
                    return;
                },
            };

            if is_this_exist_date(year_num, month_num, day_num) == -1 {
                return;
            }

            let new_profile_data = Profile {
                id: id_num,
                name: new_profile[1].to_string(),
                date: Date {
                    year: year_num,
                    month: month_num,
                    day: day_num,
                },
                place: new_profile[3].to_string(),
                data: new_profile[4].to_string(),
            };
            profile_data.push(new_profile_data);
        } else {
            println!("incorrect format");
        }
    } else {
        println!("incorrect format");
    }
}

fn get_line() -> String
{
    let mut line = String::new();

    io::stdin().read_line(&mut line).expect("Failed to read line");

    line
}

fn parse_line(line: String, profile_data: &mut Vec<Profile>)
{
    let mut ch_iter = line.chars();
    let ch1 = ch_iter.next().unwrap();

    if ch1 == '%' {
        let mut cmd_split = line.split_whitespace();
        let cmd = cmd_split.next().unwrap();
        let param = cmd_split.next();
        let over_arg = cmd_split.next();
        if over_arg == None{
            if param == None {
                exec_command1(cmd, profile_data);
            } else {
                exec_command2(cmd, param.unwrap(), profile_data);
            }
        } else {
            println!("too many arg");
        }
    } else {
        new_profile(line, profile_data);
    }
}

fn main()
{
    let mut profile_data: Vec<Profile> = Vec::with_capacity(PROFILE_VEC_SIZE);

    let mut line;
    loop {
        line = get_line();
        line = line.replace("\n", "");
        parse_line(line, &mut profile_data);
    }

}
