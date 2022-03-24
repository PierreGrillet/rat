use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process::exit;

#[derive(Debug)]    // Enabling print macro
#[derive(Default)]  // macro to set default value during init
struct Options {
    number: bool,           // -n
    number_nonblank: bool,  // -b overrides number (-n)
    show_ends: bool,        // -E
    show_tabs: bool,        // -T
    show_nonprinting: bool, // -v
    squeeze_blank: bool,    // -s
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let (options, filenames) = parse_config(&args);

    for file in filenames {
        let mut file = File::open(file)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let mut line_count = 0;
        let mut number_blank = 0;

        for line in contents.lines() {
            let mut l = line.to_string();

            if line == "" {
                number_blank += 1;
            } else {
                number_blank = 0;
            }

            if options.squeeze_blank && number_blank > 1 {
                continue;
            }

            if options.show_tabs {
                l = l.replace("\t","^I");
            }

            if options.show_nonprinting {

                let line = l.clone().into_bytes();
                let mut new: Vec<u8> = Vec::new();

                for i in 0..line.len() {
                    if line[i] as char == '\t'
                    || line[i] as char == '\n' {
                        new.push(line[i]);
                    } else {
                        convert_nonprinting(line[i], &mut new);
                    }
                }

                let res = String::from_utf8(new);
                if res.is_ok(){
                    l = res.unwrap();
                }
            }


            if options.number && ! options.number_nonblank {
                line_count += 1;
                l.insert_str(0, &format!("{: >6}  ", line_count))
            }

            if options.number_nonblank {
                if line != "" {
                    line_count += 1;
                    l.insert_str(0, &format!("{: >6}  ", line_count))
                } else {
                    l.insert_str(0, "        ")
                }
            }

            if options.show_ends {
                l.insert(l.chars().count(), '$');
            }

            println!("{}", l);

        }
    }
    Ok(())
}

fn parse_config(args: &[String]) -> (Options, Vec<String>) {
    let mut filenames : Vec<String> = Vec::new();
    let mut options = Options{ ..Default::default() }; //Default is false

    for arg in &args[1..] {

        if arg.starts_with("--") {
            let opt = &arg[..]; //convert String to std::str
            match opt {
                "--show-all" => {
                    options.show_nonprinting = true;
                    options.show_ends = true;
                    options.show_tabs = true;
                },
                "--number-nonblank"  => options.number_nonblank = true,
                "--show-ends"        => options.show_ends = true,
                "--number"           => options.number = true,
                "--squeeze-blank"    => options.squeeze_blank = true,
                "--show-tabs"        => options.show_tabs = true,
                "--show-nonprinting" => options.show_nonprinting = true,
                "--help"    => help(),
                "--version" => version(),

                _ => { // Default Case :
                    println!("cat : invalid option -- '{}'", arg);
                    exit(1);
                }
            }
        } else if arg.starts_with("-") {
            for c in arg[1..].chars() {
                match c {
                    'A' => {
                        options.show_nonprinting = true;
                        options.show_ends = true;
                        options.show_tabs = true;
                    },
                    'b' => {
                        options.number_nonblank = true;
                    },
                    'e' => {
                        options.show_nonprinting = true;
                        options.show_ends = true;
                    },
                    'E' => {
                        options.show_ends = true;
                    },
                    'n' => {
                        options.number = true;
                    },
                    's' => {
                        options.squeeze_blank = true;
                    },
                    't' => {
                        options.show_nonprinting = true;
                        options.show_tabs = true;
                    },
                    'T' => {
                        options.show_tabs = true;
                    },
                    'u' => {}, // ignored
                    'v' => {
                        options.show_nonprinting = true;
                    },
                    _ => {
                        println!("cat : invalid option -- '{}'", arg);
                        exit(1);
                    }
                }
            }
        } else { // if no "-" then it must be a filename
            filenames.push(arg.to_string())
        }
    }
    /*return*/ (options, filenames)
}

fn convert_nonprinting(c : u8, str : &mut Vec<u8>) {
    if c < 128 {
        if c > 31 && c < 127 {
            str.push(c); // it's a regular char
        } else {
            str.push('^' as u8);
            str.push((c + 64) % 128);
        }
    } else {
        str.push('M' as u8);
        str.push('-' as u8);
        convert_nonprinting(c % 128, str);
    }
}

fn _old_voption(){
    // print!("i=[");
    // for (i, c) in line.char_indices() {
    //     print!("{},",i);
    //     if c.is_ascii() {
    //         if c < ' ' && c != '\t' && c != '\n' || c as u32 == 127 {
    //             let t = ((c as u32) + 64) % 128;
    //             l.insert(i + offset, char::from_u32(t).unwrap());
    //             l.insert(i + offset, '^');
    //             l.remove(i + offset);
    //             offset += 1;
    //         }
    //     } else { //non ascii characters
    //         let c_str = c.to_string();
    //
    //         l.remove(i);
    //         // i = if i > 0 { i - 1 } else { i };
    //
    //         // println!("{:}", l);
    //         // print!("{}=[len={}, bytes=[", c, c.len_utf8());
    //         for _b in c_str.as_bytes() {
    //             //print!("{}/{},", b, '?');//char::from_u32((*b % 128) as u32).unwrap());
    //
    //             // l.insert(i + offset, '?'); //char::from_u32((*b % 128) as u32).unwrap());
    //             // l.insert(i, '?');
    //             l.insert(i, '-');
    //             l.insert(i, 'M');
    //         }
    //         // offset -= 1;
    //         // print!("] ],");
    //     }
    // }
    // print!("]\n");
}

fn help() {
    println!("Usage: rat [OPTION]... [FILE]...
Concatenate FILE(s) to standard output.

  -A, --show-all           equivalent to -vET
  -b, --number-nonblank    number nonempty output lines, overrides -n
  -e                       equivalent to -vE
  -E, --show-ends          display $ at end of each line
  -n, --number             number all output lines
  -s, --squeeze-blank      suppress repeated empty output lines
  -t                       equivalent to -vT
  -T, --show-tabs          display TAB characters as ^I
  -u                       (ignored)
  -v, --show-nonprinting   use ^ and M- notation, except for LFD and TAB
      --help     display this help and exit
      --version  output version information and exit
");
exit(0)
}


fn version() {
    println!("rat v0.1 written in rust by Pierre Grillet (C)2022");
    exit(0);
}
