use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::BufRead;
extern crate clap;
use clap::{Arg, App};
use termion;
use std::cmp::min;

fn push_line_to_buble_buffer(buffer: &mut String, text: &str, padding: &str, tw: usize) {    
    buffer.push_str(padding);
    buffer.push_str("< ");
    buffer.push_str(text);
    let pad = tw-(text.len()+3);
    for _ in 0..pad {
        buffer.push(' ');
    }
    buffer.push_str(" >\n");
}

fn generate_buble(pos: i32, text: &str) -> String {
    
    let terminal_size = match termion::terminal_size() {Ok(x) => Some(x), Err(_) => None};
    
    let terminal_w = terminal_size.map(|x| x.0);
    
    let mut padding = String::new();
    for _i in 1..pos {
        padding.push(' ');
    }
    
    let tw = match terminal_w {
        Some(w) => min(text.len() + 4,w as usize - padding.len() - 1),
        None => text.len()
    };
    
    let mut buffer = String::new();
    
    buffer.push_str(" ");
    for _i in 0..(tw-2) {
        buffer.push('_');
    }
    buffer.push('\n');
    
    let mut pos = 0;

    match terminal_w {
        Some(w) => {
            while pos < text.len() {
                let w_overhead = (padding.len() + 4) as usize;
                let len = min(text.len()-pos, w as usize - w_overhead);
                push_line_to_buble_buffer(&mut buffer,&text[pos..(pos+len)],&padding,tw);
                pos += len
            }
        },
        None => push_line_to_buble_buffer(&mut buffer,text,&padding,tw),
    }
    
    buffer.push_str(&*padding);
    buffer.push_str(" ");
    for i in 0..(tw-2) {
        buffer.push('-');
    }
    
    buffer
}

fn format_var(name: &str, pos: i32,text: &str) -> String {
    match name {
        "thinking" => generate_buble(pos,text),
        "tonge" => " U".to_string(),
        e => panic!("var {}{}{} not implemented",'{',e,'}')
    }
} 

fn format_cow_line(line: &str,text: &str) -> String {
    let mut chars = line.chars();
    let mut buffer = String::new();
    let mut col_counter: i32 = 0;
    'line_eater: loop {
        col_counter += 1;
        match chars.next() {
            Some(cchar) => match cchar {
                '$'  => {
                    match chars.next().expect("EOL after $") {
                        '{' => {
                            let mut varname = String::new();
                            
                            'name_eater: loop {
                                match chars.next().expect("unclosed '${'") {
                                    '}' => break 'name_eater,
                                    x =>  varname.push(x)
                                }
                            }
                            
                            buffer.push_str(&*format_var(&*varname, col_counter, text));
                        }
                        _ => panic!("no var after $")
                    }
                },
                '\\' => match chars.next() {
                    None => panic!("EOL after '\\'"),
                    Some(x) => buffer.push(x),
                },
                x => buffer.push(x)
            },
            None => break 'line_eater
        }
    };
    buffer
}

fn main() {

        let matches = App::new("Rust cow")
                          .version("1.0")
                          .author("Maurycy Z. <10maurycy10@gmail.com>")
                          .about("makes askii art cows")
                          .arg(Arg::with_name("cowfile")
                               .short("f")
                               .long("cowfile")
                               .value_name("FILE")
                               .help("Sets cowfile, these contain the template for the output.")
                               .takes_value(true))
                          .get_matches();

    let cowfile_path = matches.value_of("cowfile").unwrap_or("default.rcow");

    let cowfile = File::open(cowfile_path).expect(&*format!("cant read {}",cowfile_path));
    let mut lines = BufReader::new(cowfile).lines().map(|x| x.unwrap());
    
    let stdin = io::stdin();
    let text = stdin.lock().lines().next().expect("no input").expect("no input");
    
    loop {
        match lines.next() {
            Some(line) => match &*line {
                "---<COW-BLOCK>---" => loop {
                    match lines.next() {
                        Some(line) => match &*line {
                            "--->COW-BLOCK<---" => break,
                            line => println!("{}",format_cow_line(line,&text)),
                        },
                        None => panic!("unclosed COW-BLOCK")
                    }
                },
                _ => ()
            },
            None => break
        }
    }
}
