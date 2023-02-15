//  SPDX-FileCopyrightText: Copyright 2022-2023 James M. Putnam (putnamjm.design@gmail.com)
//  SPDX-License-Identifier: MIT

//! runtime loader/repl
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
    Debug,
    Eval,
    Load,
    Pipe,
    Quiet,
    Script,
}

fn options(mut argv: Vec<String>) -> Option<Vec<OptDef>> {
    let mut opts = getopt::Parser::new(&argv, "h?psdvc:e:l:q:");
    let mut optv = Vec::new();

    loop {
        let opt = opts.next().transpose();
        match opt {
            Err(_) => {
                if let Err(error) = opt {
                    eprintln!("runtime: option {error:?}")
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
                        print!("runtime: {} ", <Mu as Core>::VERSION);
                        return None;
                    }
                    Opt('p', None) => {
                        optv.push((OptType::Pipe, String::from("")));
                    }
                    Opt('d', None) => {
                        optv.push((OptType::Debug, String::from("")));
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
    eprintln!("runtime: {}: [-h?psvcelq] [file...]", <Mu as Core>::VERSION);
    eprintln!("?: usage message");
    eprintln!("h: usage message");
    eprintln!("c: [name:value,...]");
    eprintln!("d: debugging on");
    eprintln!("e: eval [form] and print result");
    eprintln!("l: load [path]");
    eprintln!("p: pipe mode");
    eprintln!("q: eval [form] quietly");
    eprintln!("s: script mode");
    eprintln!("v: print version and exit");

    std::process::exit(0);
}

fn load(mu: &Mu, path: &str, debug: bool) -> Option<()> {
    let about = fs::metadata(path).ok()?;

    if !about.is_file() {
        return None;
    }

    let load_form = "(mu:open :file :input \"".to_string() + path + "\")";
    let istream = mu.eval(mu.read_string(load_form).unwrap()).unwrap();
    let eof_value = mu.read_string(":eof".to_string()).unwrap(); // need make_symbol here

    #[allow(clippy::while_let_loop)]
    loop {
        match mu.read(istream, true, eof_value) {
            Ok(form) => {
                if mu.eq(form, eof_value) {
                    break;
                }

                if debug {
                    print!("{path}: ");
                    mu.write(form, true, mu.stdout).unwrap();
                    print!(",");
                }

                match mu.compile(form) {
                    Ok(form) => {
                        if debug {
                            mu.write(form, true, mu.stdout).unwrap();
                            print!(",");
                        }

                        match mu.eval(form) {
                            Ok(eval) => {
                                if debug {
                                    mu.write(eval, true, mu.stdout).unwrap();
                                    println!();
                                }
                            }
                            Err(_) => {
                                break;
                            }
                        }
                    }
                    Err(_) => {
                        break;
                    }
                }
            }
            Err(_) => {
                break;
            }
        }
    }

    Some(())
}

pub fn main() {
    let mut config = String::new();
    let mut debug = false;
    let mut pipe = false;
    let mut script = false;

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
                    OptType::Debug => {
                        debug = true;
                    }
                    OptType::Script => {
                        script = true;
                    }
                    OptType::Pipe => {
                        pipe = true;
                    }
                    OptType::Load => match load(&mu, &opt.1, debug) {
                        Some(_) => (),
                        None => {
                            eprintln!("runtime: failed to load {}", &opt.1);
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
                "runtime: v{}; config [{}]",
                <Mu as Core>::VERSION,
                if mu.config.is_empty() { "" } else { &mu.config },
            );
        }

        let eof_value = mu.read_string(":eof".to_string()).unwrap(); // need make_symbol here

        loop {
            if !pipe {
                mu.write_string("mu> ".to_string(), mu.stdout).unwrap();
                std::io::stdout().flush().unwrap();
            }

            match mu.read(mu.stdin, true, eof_value) {
                Ok(tag) => {
                    if mu.eq(tag, eof_value) {
                        break;
                    }

                    #[allow(clippy::single_match)]
                    match mu.compile(tag) {
                        Ok(form) => match mu.eval(form) {
                            Ok(eval) => mu.write(eval, false, mu.stdout).unwrap(),
                            Err(_) => (),
                        },
                        Err(_) => (),
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
