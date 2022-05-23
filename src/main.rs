// Recursively finds files with given permissions/name in a directory.

use std::{
    env::args,
    fs::read_dir,
    io::{Error, ErrorKind::InvalidInput, Result},
    os::unix::fs::PermissionsExt,
    path::Path,
};

// TODO: Should I suppress non-fatal errors?
fn find_name(dir: &Path, name: &String, exact: bool) -> Result<()> {
    for entry in read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() && !path.is_symlink() {
            match find_name(&path, name, exact) {
                Ok(_) => (),
                Err(_) => (),
            }
        } else if path.is_file() {
            let cur_name = path.file_name().unwrap().to_str().unwrap();
            if (exact && cur_name == name) || cur_name.contains(name) {
                println!("{}", path.display())
            }
        }
    }
    Ok(())
}

// fn walk_dir(dir: &Path) -> Result<()> {
//     for entry in read_dir(dir)? {
//         let path = entry?.path();
//         if path.is_dir() && !path.is_symlink() {
//             match walk_dir(&path) {
//                 Ok(_) => (),
//                 Err(_) => (),
//             }
//         } else if path.is_file() {
//             println!("{}", path.display())
//         }
//     }
//     Ok(())
// }

// fn find_max<'a>(dir: &Path, max: &u64, max_file: &'a Path) -> Result<&'a Path> {
//     for entry in read_dir(dir)? {
//         let path = entry?.path();
//         if path.is_dir() && !path.is_symlink() {
//             find_max(&path, max, max_file);
//         } else if path.is_file() {
//             let size = path.metadata()?.len();
//             if size > *max {
//                 *max = size;
//                 *max_file = path;
//             }
//         }
//     }
//     Ok(max_file)
// }

fn validate_perms(path: &Path, perm_bytes: &str) -> Result<()> {
    // Validate that the given permissions string is valid.
    if perm_bytes.len() != 9 {
        return Err(Error::new(
            InvalidInput,
            "Permissions string must be 9 characters long.",
        ));
    }
    // Create a bitmask of the permissions.
    let mut perms = 1 << 15;
    for (i, (p, v)) in perm_bytes.chars().zip("rwxrwxrwx".chars()).enumerate() {
        if p == v {
            perms |= 1 << (8 - i);
        } else if p != '-' {
            return Err(Error::new(
                InvalidInput,
                "Permissions string must be of the form: rwxrwxrwx.",
            ));
        }
    }
    fn find_perms(dir: &Path, perms: u32) -> Result<()> {
        for entry in read_dir(dir)? {
            let path = entry?.path();
            if path.is_dir() && !path.is_symlink() {
                match find_perms(&path, perms) {
                    Ok(_) => (),
                    Err(_) => (),
                }
            } else if path.metadata()?.permissions().mode() == perms {
                println!("{}", path.display())
            }
        }
        Ok(())
    }
    // Find files at the current path with the requested permissions.
    Ok(find_perms(&path, perms)?)
}

fn main() -> Result<()> {
    let args: Vec<String> = args().collect();

    if args.len() != 6 {
        println!(
            "Usage: {} [permissions, name] [of, with] <query> in <directory>",
            args[0]
        );
        return Ok(());
    }

    let path = Path::new(&args[5]).canonicalize()?;
    match args[1].as_str() {
        "permissions" => validate_perms(&path, &args[3]),
        "name" => match args[2].as_str() {
            "of" => find_name(&path, &args[3], true),
            "with" => find_name(&path, &args[3], false),
            _ => Err(Error::new(
                InvalidInput,
                "Must specify 'of' or 'with' after 'name'.",
            )),
        },
        // "size" => Ok(println!("{:#?}", find_max(&path, &0, &path))),
        _ => Err(Error::new(
            InvalidInput,
            "Must specify 'permissions' or 'name' before 'of' or 'with'.",
        )),
    }
}
