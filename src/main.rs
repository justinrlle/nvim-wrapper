use std::{
    env,
    process::{exit, Command},
};

fn main() {
    let mut args = env::args().peekable();
    args.next();
    let prog_name = match args.peek() {
        Some(opt) if opt == "--gui" => {
            args.next();
            "nvim-qt"
        }
        _ => "nvim",
    };
    let res = Command::new(prog_name)
        .args(args)
        .env_remove("LC_CTYPE")
        .spawn()
        .expect("failed to spawn binary")
        .wait()
        .expect("failed running binary");
    if !res.success() {
        if let Some(code) = res.code() {
            exit(code);
        }
    }
}
