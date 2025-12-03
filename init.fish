#!/usr/bin/fish

set template "use std::io;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    for line in io::stdin().lines() {
        let line = line.expect(\"Could not read line\");
    }

    Ok(())
}"

set dir "aoc-$argv[1]"
mkdir $dir
cd $dir

cargo init
echo $template >src/main.rs
touch src/lib.rs
