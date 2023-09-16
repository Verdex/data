
mod parsing;

use std::fs;
use std::path::PathBuf;

// TODO : check to see if there's a .gitignore and then check that to see which files should be ignored
// TODO : flag to ignore the .gitignore
// TODO : flag to go through .git directory
// TODO : flag to process only the file from the given path
// TODO : flag to process only stdin (and I guess a way to specify how to parse?)
// TODO : will need to ignore unknown file extensions

fn main() -> std::io::Result<()> {
    let mut work : Vec<PathBuf> = vec![".".into()];
    while work.len() != 0 {

        for x in fs::read_dir(work.pop().unwrap())? {
            let x = x?;
            let ft = x.file_type()?;
            if ft.is_dir() {
                work.push(x.path());
            }
            else if ft.is_file() {
                println!("{:?}", x);
            }
        }
        
    }

    Ok(())
}
