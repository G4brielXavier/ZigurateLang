use std::fs::{self, File};
use memmap2::Mmap;
use std::path::{PathBuf, Path};

#[derive(Debug)]
pub struct Sita {}

impl Sita {
    
    pub fn gibil() -> Self {
        Self {}
    }


    pub fn get_dir_by_name<P: AsRef<Path>>(&self, dirname: &str, path: P) -> Result<PathBuf, String> {

        let path_ref = path.as_ref();

        let content_dir = fs::read_dir(path_ref)
            .map_err(|e| format!("{:?}", e))?;

        for entry in content_dir.flatten() {
            let child_path = entry.path();

            if child_path.is_dir() {
                
                if child_path.file_name().and_then(|s| s.to_str()) == Some(dirname) {
                    return Ok(child_path)
                }

                if let Ok(found) = self.get_dir_by_name(dirname, &child_path) {
                    return Ok(found)
                }

            }
        }

        return Err(format!("{} not found", dirname))

    }



    pub fn get_file_by_name<P: AsRef<Path>>(&self, filename: &str, path: P) -> Result<PathBuf, String> {

        let content_dir = fs::read_dir(&path)
            .map_err(|e| format!("{:?}", e))?;

        for entry in content_dir.flatten() {
            
            let child_path = entry.path();

            if child_path.is_file() {

                if child_path.file_name().and_then(|s| s.to_str()) == Some(filename) {
                    return Ok(child_path)
                }

                if let Ok(found) = self.get_file_by_name(filename, &child_path) {
                    return Ok(found)
                }

            }

        }

        return Err(format!("{} not found", filename))

    }


    
    pub fn read_file<P: AsRef<Path>>(&self, path: P) -> Result<String, String> {

        let file = File::open(path)
            .map_err(|e| e.to_string())?;

        let mmap = unsafe {
            Mmap::map(&file)
                .map_err(|e| e.to_string())?
        };

        let content = String::from_utf8(mmap.to_vec())
            .map_err(|e| e.to_string())?;

        Ok(content)

    }



    pub fn create_dir<P: AsRef<Path>>(&self, dirname: &str, path: P) -> Result<PathBuf, String> {

        let fullpath = path.as_ref().join(dirname);

        if !fullpath.exists() {
            fs::create_dir_all(&fullpath)
                .map_err(|e| e.to_string())?;

            return Ok(fullpath)
        }

        return Ok(fullpath)

    }



    pub fn create_file<P: AsRef<Path>>(&self, filename: &str, path: P) -> Result<PathBuf, String> {

        let fpath = path.as_ref().join(filename);

        if !fpath.exists() {
            File::create(&fpath)
                .map_err(|e| format!("{:?}", e))?;

            return Ok(fpath)
        }

        return Ok(fpath)

    }



    pub fn edit_file<P: AsRef<Path>>(&self, path: P, content: String) -> Result<(), String> {

        fs::write(path, content)
            .map_err(|e| e.to_string())?;

        Ok(())

    }



    pub fn current_dir(&self) -> Result<PathBuf, String> {
        match env::current_dir() {
            Ok(path) => return Ok(path),
            Err(e) => return Err(format!("{}", e))
        }
    }



    pub fn path_exists<P: AsRef<Path>>(&self, path: P) -> Result<bool, String> {
        let fullpath = path.as_ref();
        Ok(fullpath.exists())
    }


}