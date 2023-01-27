//  SPDX-FileCopyrightText: Copyright 2022-2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! mu-runtime loader/repl
extern crate mu;

use {
    crate::mu::core::mu::{Core, Mu, MuCondition},
    getopt::Opt,
    std::{fs, io::Write},
};

// options
type OptDef = (OptType, String);

#[derive(Debug, PartialEq)]
enum OptType {
    Config,
    Eval,
    Load,
    Pipe,
    Quiet,
    Script,
}

fn options(mut argv: Vec<String>) -> Option<Vec<OptDef>> {
    let mut opts = getopt::Parser::new(&argv, "h?psvc:e:l:q:");
    let mut optv = Vec::new();

    loop {
        let opt = opts.next().transpose();
        match opt {
            Err(_) => {
                if let Err(error) = opt {
                    eprintln!("mu-runtime: option {error:?}")
                };
                usage();
                std::process::exit(0);
            }
            Ok(None) => {
                break;
            }
            Ok(clause) => match clause {
                Some(opt) => match opt {
                    Opt('h', None) | Opt('?', None) => usage(),
                    Opt('v', None) => {
                        print!("mu-runtime: {} ", <Mu as Core>::VERSION);
                        return None;
                    }
                    Opt('p', None) => {
                        optv.push((OptType::Pipe, String::from("")));
                    }
                    Opt('s', None) => {
                        optv.push((OptType::Script, String::from("")));
                    }
                    Opt('e', Some(expr)) => {
                        optv.push((OptType::Eval, expr));
                    }
                    Opt('q', Some(expr)) => {
                        optv.push((OptType::Quiet, expr));
                    }
                    Opt('l', Some(path)) => {
                        optv.push((OptType::Load, path));
                    }
                    Opt('c', Some(config)) => {
                        optv.push((OptType::Config, config));
                    }
                    _ => {
                        usage();
                    }
                },
                None => panic!(),
            },
        }
    }

    for file in argv.split_off(opts.index()) {
        optv.push((OptType::Load, file));
    }

    Some(optv)
}

fn usage() {
    eprintln!(
        "mu-runtime: {}: [-h?psvcelq] [file...]",
        <Mu as Core>::VERSION
    );
    eprintln!("?: usage message");
    eprintln!("h: usage message");
    eprintln!("c: [name:value,...]");
    eprintln!("e: eval [form] and print result");
    eprintln!("l: load [path]");
    eprintln!("p: pipe mode");
    eprintln!("q: eval [form] quietly");
    eprintln!("s: script mode");
    eprintln!("v: print version and exit");

    std::process::exit(0);
}

fn load(mu: &Mu, path: &str) -> Option<()> {
    let about = fs::metadata(path).ok()?;

    if !about.is_file() {
        return None;
    }

    let load_form = "(mu:open :file :input \"".to_string() + path + "\")";
    let istream = mu.eval(mu.read_string(load_form).unwrap()).unwrap();
    let eof_value = mu.read_string(":eof".to_string()).unwrap(); // need make_symbol here

    loop {
        let form = mu.read(istream, true, eof_value).unwrap();

        if mu.eq(form, eof_value) {
            break;
        }

        match mu.compile(form) {
            Ok(form) => match mu.eval(mu.compile(form).unwrap()) {
                Ok(_) => (),
                Err(_) => {
                    return None;
                }
            },
            Err(_) => {
                return None;
            }
        }
    }

    Some(())
}

pub fn main() {
    let mut script = false;
    let mut pipe = false;
    let mut config = String::new();

    match options(std::env::args().collect()) {
        Some(opts) => {
            for opt in opts {
                if opt.0 == OptType::Config {
                    config = opt.1;
                }
            }
        }
        None => std::process::exit(0),
    }

    let mu = <Mu as Core>::new(config);

    match options(std::env::args().collect()) {
        Some(opts) => {
            for opt in opts {
                match opt.0 {
                    OptType::Eval => {
                        if let Ok(ptr) = mu.read_string(opt.1) {
                            let form = mu.compile(ptr).unwrap();
                            if let Ok(eval) = mu.eval(form) {
                                mu.write(eval, true, mu.stdout).unwrap();
                                println!();
                            }
                        }
                    }
                    OptType::Script => {
                        script = true;
                    }
                    OptType::Pipe => {
                        pipe = true;
                    }
                    OptType::Load => match load(&mu, &opt.1) {
                        Some(_) => (),
                        None => {
                            eprintln!("mu-runtime: failed to load {}", &opt.1);
                            std::process::exit(0);
                        }
                    },
                    OptType::Quiet => {
                        if let Ok(ptr) = mu.read_string(opt.1) {
                            let form = mu.compile(ptr).unwrap();
                            mu.eval(form).unwrap();
                        }
                    }
                    OptType::Config => (),
                }
            }
        }
        None => std::process::exit(0),
    };

    if !script {
        if !pipe {
            println!(
                "mu-runtime: v{}; config [{}]",
                <Mu as Core>::VERSION,
                if mu.config.is_empty() { "" } else { &mu.config },
            );
        }

        loop {
            if !pipe {
                mu.write_string("mu> ".to_string(), mu.stdout).unwrap();
                std::io::stdout().flush().unwrap();
            }

            match mu.read(mu.stdin, false, mu.nil()) {
                Ok(ptr) => {
                    let form = mu.compile(ptr).unwrap();
                    if let Ok(eval) = mu.eval(form) {
                        mu.write(eval, false, mu.stdout).unwrap();
                    }
                }
                Err(e) => {
                    if let MuCondition::Eof = e.condition {
                        std::process::exit(0);
                    }
                }
            }

            println!();
        }
    }
}