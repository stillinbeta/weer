use clap::{Command, command, ArgAction, arg};

pub fn build<'a>() -> Command<'a> {
    command!()
        .args(&[
            arg!(--city [name])
                .conflicts_with_all(&[
                    "ip",
                    "coords"
                ]),
            arg!(--ip [ip_addr])
                .conflicts_with_all(&[
                    "city",
                    "coords"
                ]),
            arg!(--coords [coords])
                .conflicts_with_all(&[
                    "city",
                    "ip"
                ])
        ])
        .arg_required_else_help(true)
        .subcommands(applet_commands())
}

fn applet_commands<'a>() -> [Command<'a>; 4] {
    [
        Command::new("search"),
        forecast(),
        history(),
        Command::new("future")
            .arg(arg!(--dt <date>)),
    ]
}

fn forecast<'a>() -> Command<'a> {
    Command::new("forecast")
        .args(&[
            arg!(--dt [date]),
            arg!(-d --days <num>),
            arg!(--hour <hour>),
            arg!(--aqi)
                .action(ArgAction::SetTrue),
            arg!(--alerts)
                .action(ArgAction::SetTrue),
        ])
}

fn history<'a>() -> Command<'a> {
    Command::new("history")
        .args(&[
            arg!(--dt <date>),
            arg!(--end_dt [date]),
            arg!(-h --hour [hour]),
        ])
}
