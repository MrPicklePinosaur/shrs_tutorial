use shrs::prelude::*;

fn main() {
    let myshell = ShellBuilder::default()
        .build()
        .unwrap();

    myshell.run();
}
