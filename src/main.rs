use std::{
    fs::read_link,
    path::{self, Path, PathBuf},
};

pub struct ReadLink {
    input_path: PathBuf,
}

impl ReadLink {
    pub fn new(&mut self, path: &Path) -> Self {
        return ReadLink{
            input_path: PathBuf::from(path)
        }
    }

    pub fn follow_link(&self) -> Result<PathBuf, std::io::Error> {
        let current_dir = std::env::current_dir()?;

        let input_path_prefix = self.input_path.parent().unwrap_or(Path::new(""));

        let mut path_current = PathBuf::from(&self.input_path); // initial read linked value
        let mut path_previous;

        while !path_current.as_os_str().is_empty() {
            path_previous = path_current.clone();

            if path_current.is_relative() {
                if path_current.starts_with(".") {
                    path_current = current_dir.join(path_current);
                } else {
                    path_current = input_path_prefix.join(path_current);
                };
            } else {
                path_current = read_link(path_current)?;
            }

            path_current = path::absolute(path_current)?;

            if path_previous == path_current
                && path_current.is_symlink()
                && path_previous.is_symlink()
            {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Circular symlink is not allowed!",
                ));
            }
        }

        let abs_path = path::absolute(path_current)?;
        return Ok(abs_path);
    }
}



fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_symlink_against_file_absolute_path() {}

    #[test]
    fn test_symlink_against_file_relative_path_1() {}

    #[test]
    fn test_symlink_against_file_relative_path_2() {}

    #[test]
    fn test_symlink_against_symlink_relative_path() {}

    #[test]
    fn test_symlink_against_symlink_absolute_path() {}

    #[test]
    fn test_against_circular_symlink() {}

    
}
