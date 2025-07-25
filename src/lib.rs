use std::{
    fs::read_link,
    io,
    path::{self, Path, PathBuf},
};

pub struct SimpleReadLink {
    input_path: PathBuf,
}

impl SimpleReadLink {
    pub fn from(path: &Path) -> Self {
        return SimpleReadLink {
            input_path: PathBuf::from(path),
        };
    }

    pub fn follow_link(&self) -> Result<PathBuf, std::io::Error> {
        let input_path = path::absolute(&self.input_path)?; // get the absolute path to avoid bugs with parent() method

        let input_path_parent = input_path.parent().unwrap_or(Path::new(""));

        let mut first_run = true;
        let mut path_current: PathBuf = PathBuf::default();

        let max_loop = 30; // there must be something wrong with you if you want to increase the amount of readlinks beyond this
        let mut counter = 0;

        loop {
            if first_run {
                path_current = read_link(&self.input_path)?;
                first_run = false;
            } else {
                path_current = read_link(path_current)?;
            }

            if path_current.is_relative() {
                path_current = input_path_parent.join(path_current);
            }

            counter = counter + 1;

            if counter > max_loop {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Maximum read_links exceeded!",
                ));
            } else if !path_current.is_symlink() {
                break;
            }
        }

        let abs_path = path::absolute(path_current)?;
        return Ok(abs_path);
    }
}


#[cfg(test)]
mod tests {
    use crate::SimpleReadLink;
    use std::path::PathBuf;

    const SANDBOX_DIR: &str = "/tmp/sandbox";

    #[test]
    fn test_readlink_against_normal_file() {
        let input_path = PathBuf::from(SANDBOX_DIR).join("test_file.txt");
        let result = SimpleReadLink::from(&input_path).follow_link();
        assert!(result.is_err());
    }

    #[test]
    fn test_readlink_symlink_against_file_absolute_path() {
        let input_path = PathBuf::from(SANDBOX_DIR).join("symlink_file_absolute");
        let result = SimpleReadLink::from(&input_path).follow_link();

        let actual = PathBuf::from(SANDBOX_DIR).join("test_file.txt");
        println!("input_path: {:?}", input_path);
        println!("RESULT: {:?}", result);
        assert_eq!(result.unwrap(), actual);
    }

    #[test]
    fn test_readlink_symlink_against_file_relative_path_1() {
        let input_path = PathBuf::from(SANDBOX_DIR)
            .join("tmp")
            .join("test_folder")
            .join("test_relative.sym.relative_1");
        let result = SimpleReadLink::from(&input_path).follow_link();

        let actual = PathBuf::from(SANDBOX_DIR)
            .join("tmp")
            .join("test_folder")
            .join("test_relative.txt");
        println!("input_path: {:?}", input_path);
        println!("RESULT: {:?}", result);
        assert_eq!(result.unwrap(), actual);
    }

    #[test]
    fn test_readlink_symlink_against_file_relative_path_2() {
        let input_path = PathBuf::from(SANDBOX_DIR)
            .join("tmp")
            .join("test_folder")
            .join("test_relative.sym.relative_2");
        let result = SimpleReadLink::from(&input_path).follow_link();

        let actual = PathBuf::from(SANDBOX_DIR)
            .join("tmp")
            .join("test_folder")
            .join("test_relative.txt");
        println!("input_path: {:?}", input_path);
        println!("RESULT: {:?}", result);
        assert_eq!(result.unwrap(), actual);


    }

    #[test]
    fn test_readlink_symlink_against_symlink_relative_path() {
        let input_path = PathBuf::from(SANDBOX_DIR)
            .join("symlink_jump_3_rel_a");
        let result = SimpleReadLink::from(&input_path).follow_link();

        let actual = PathBuf::from(SANDBOX_DIR)
            .join("tmp")
            .join("test_folder")
            .join("test_relative.txt");
        println!("input_path: {:?}", input_path);
        println!("RESULT: {:?}", result);
        assert_eq!(result.unwrap(), actual);
    }

    #[test]
    fn test_readlink_symlink_against_symlink_absolute_path() {
        let input_path = PathBuf::from(SANDBOX_DIR)
            .join("symlink_jump_3_rel_b");
        let result = SimpleReadLink::from(&input_path).follow_link();

        let actual = PathBuf::from(SANDBOX_DIR)
            .join("tmp")
            .join("test_folder")
            .join("test_relative.txt");
        println!("input_path: {:?}", input_path);
        println!("RESULT: {:?}", result);
        assert_eq!(result.unwrap(), actual);

    }

    #[test]
    fn test_readlink_against_circular_symlink() {
        // should not produce an infinite loop 
        let input_path = PathBuf::from(SANDBOX_DIR).join("circular_c");
        let result = SimpleReadLink::from(&input_path).follow_link();
        assert!(result.is_err());
    }

    #[test]
    fn test_readlink_against_hidden_files() {
        let input_path = PathBuf::from(SANDBOX_DIR)
            .join("symlink_jump_3_rel_b");
        let result = SimpleReadLink::from(&input_path).follow_link();

        let actual = PathBuf::from(SANDBOX_DIR)
            .join("tmp")
            .join("test_folder")
            .join("test_relative.txt");
        println!("input_path: {:?}", input_path);
        println!("RESULT: {:?}", result);
        assert_eq!(result.unwrap(), actual);
    }

    // Test against root files
    #[test]
    fn test_readlink_against_files_in_root_abs() {
        let input_path = PathBuf::from("/test_root_symlink_abs");
        let result = SimpleReadLink::from(&input_path).follow_link();

        let actual = PathBuf::from("/test_root_file");
        println!("input_path: {:?}", input_path);
        println!("RESULT: {:?}", result);
        assert_eq!(result.unwrap(), actual);
    }


    // Test against root files
    #[test]
    fn test_readlink_against_files_in_root_rel() {
        let input_path = PathBuf::from("/test_root_symlink_rel");
        let result = SimpleReadLink::from(&input_path).follow_link();

        let actual = PathBuf::from("/test_root_file");
        println!("input_path: {:?}", input_path);
        println!("RESULT: {:?}", result);
        assert_eq!(result.unwrap(), actual);
    }

}
