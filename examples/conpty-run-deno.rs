use std::ffi::OsString;

use winptyrs::{AgentConfig, MouseMode, PTYArgs, PTYBackend, PTY};

fn main() {
    // see [ANSI Escape Codes ~ Discussion](https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797)
    // see [ANSI Escape Codes](https://en.wikipedia.org/wiki/ANSI_escape_code)
    // see [Python ANSI Escape Code Regex](https://www.tutorialspoint.com/How-can-I-remove-the-ANSI-escape-sequences-from-a-string-in-python) @@ <https://archive.is/4On3c>
    // see [ansi-regex (JS)](https://github.com/chalk/ansi-regex)
    //  ... const pattern = ['[\\u001B\\u009B][[\\]()#;?]*(?:(?:(?:(?:;[-a-zA-Z\\d\\/#&.:=?%@~_]+)*|[a-zA-Z\\d]+(?:;[-a-zA-Z\\d\\/#&.:=?%@~_]*)*)?\\u0007)','(?:(?:\\d{1,4}(?:;\\d{0,4})*)?[\\dA-PR-TZcf-nq-uy=><~]))'].join('|');
    let pos_ansi_codes_rex = regex::Regex::new(r"(\x9B|\x1B\[)[0-?]*[ -/]*[HfABCDEFGn]").unwrap();
    let all_ansi_codes_rex = regex::Regex::new(r"(\x9B|\x1B\[)[0-?]*[ -/]*[@-~]").unwrap();
    let seq_newlines_rex = regex::Regex::new(r"(?m)(\r?\n|\r){1,}").unwrap();
    let re_pos = pos_ansi_codes_rex;
    let re_all = all_ansi_codes_rex;
    let re_seq_newlines = seq_newlines_rex;
    let pty_args = PTYArgs {
        cols: 100,
        rows: 100,
        mouse_mode: MouseMode::WINPTY_MOUSE_MODE_NONE,
        timeout: 10000,
        // agent_config: AgentConfig::WINPTY_FLAG_COLOR_ESCAPES,
        agent_config: AgentConfig::WINPTY_FLAG_PLAIN_OUTPUT,
    };

    let max_chars_to_read = 100000;
    println!("Creating");
    let mut output = OsString::from("===start===\n");
    let exit_status;

    // FixME: all `print!(...)` after creation of PTY spawned process are not to the parent process STDOUT!
    // ... need to create the PTY process in a new thread?

    // match PTY::new_with_backend(&pty_args, PTYBackend::ConPTY) {
    match PTY::new(&pty_args) {
        Ok(mut pty) => {
            let appname = OsString::from("deno.exe");
            let args = Some(OsString::from(
                "run --reload https://deno.land/x/yargs@v17.5.1-deno/deno.ts",
            ));
            println!("{:?}", appname);
            match pty.spawn(appname, args, None, None) {
                Ok(_) => {
                    while pty.is_alive().unwrap() {
                        let _ = pty.write(OsString::from("n\r\n")).unwrap();
                        output.push(pty.read(max_chars_to_read, true).unwrap());
                        // print!(
                        //     "{}",
                        //     re_all.replace_all(
                        //         &re_pos.replace_all(output.to_str().unwrap(), "\n"),
                        //         ""
                        //     )
                        // )
                    }
                    // println!(
                    //     "{}",
                    //     re_seq_newlines.replace_all(
                    //         &re_all.replace_all(
                    //             &re_pos.replace_all(output.to_str().unwrap(), "\n"),
                    //             ""
                    //         ),
                    //         "\n"
                    //     )
                    // )
                }
                Err(err) => {
                    println!("{:?}", err);
                    panic!("{:?}", err)
                }
            }
            exit_status = pty.get_exitstatus().unwrap().unwrap();
        }
        Err(err) => {
            panic!("{:?}", err)
        }
    }
    println!("exit_status={}", exit_status);
    print!(
        "{}",
        re_seq_newlines.replace_all(
            &re_all.replace_all(&re_pos.replace_all(output.to_str().unwrap(), "\n"), ""),
            "\n"
        )
    );
}
