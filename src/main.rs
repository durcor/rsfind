// Recursively finds files with a given permission string in a directory.

use std::{
    env::args,
    fs::read_dir,
    io::{Error, ErrorKind::InvalidInput, Result},
    os::unix::fs::PermissionsExt,
    path::Path,
};

fn find(dir: &Path, perms: u32) -> Result<()> {
    for entry in read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() && !path.is_symlink() {
            match find(&path, perms) {
                Ok(_) => (),
                Err(e) => eprintln!("Warning: {}: '{}'", e, path.display()),
            }
        } else if path.metadata()?.permissions().mode() == perms {
            println!("{}", path.display());
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = args().collect();

    if args.len() != 3 {
        println!("Usage: {} <directory> <permissions string>", args[0]);
        return Ok(());
    }
    let path = Path::new(&args[1]).canonicalize()?;
    let perm_bytes = &args[2].as_bytes();
    // Validate that the given permissions string is valid.
    if perm_bytes.len() != 9 {
        return Err(Error::new(
            InvalidInput,
            "Permissions string must be 9 characters long.",
        ));
    }
    // Create a bitmask of the permissions.
    let mut perms = 1 << 15;
    for (i, (p, v)) in perm_bytes
        .iter()
        .zip("rwxrwxrwx".as_bytes().iter())
        .enumerate()
    {
        if p == v {
            perms |= 1 << (8 - i);
        } else if *p != b'-' {
            return Err(Error::new(
                InvalidInput,
                "Permissions string must be of the form: rwxrwxrwx.",
            ));
        }
    }
    // Find files at the current path with the requested permissions.
    Ok(find(&path, perms)?)
}
