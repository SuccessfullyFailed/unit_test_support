use std::sync::Mutex;
use file_ref::FileRef;
use rand::prelude::*;



const TEMP_FILE_DIR:FileRef = FileRef::new_const("target/unit_test_support/");
static mut RESERVED_FILES:Mutex<Vec<FileRef>> = Mutex::new(Vec::new());



pub struct TempFile(pub FileRef);
impl TempFile {

	/// Create a new temp file.
	pub fn new(extension:Option<&str>) -> TempFile {
		TEMP_FILE_DIR.guarantee_exists().unwrap();
		let mut file:FileRef = Self::random_file(extension);
		let reserved_files:&mut Vec<FileRef> = unsafe { RESERVED_FILES.get_mut().unwrap() };
		while reserved_files.contains(&file) {
			file = Self::random_file(extension);
		}
		reserved_files.push(file.clone());
		TempFile(file)
	}

	/// Generate a random file.
	fn random_file(extension:Option<&str>) -> FileRef {
		TEMP_FILE_DIR + &Self::random_file_name() + &extension.map(|e| format!(".{e}")).unwrap_or_default()
	}

	/// Generate a random file name.
	fn random_file_name() -> String {
		const FILE_NAME_CHARS:&str = "abcdefghijklmnopqrstuvwxyz1234567890_-.";
		const FILE_NAME_LENGTH:usize = 32;

		let mut rng:ThreadRng = rand::thread_rng();
		(0..FILE_NAME_LENGTH).map(|_| FILE_NAME_CHARS.chars().choose(&mut rng).unwrap()).collect::<String>()
	}
}
impl Drop for TempFile {
	fn drop(&mut self) {

		// Delete file.
		if self.0.exists() {
			self.0.delete().expect("Could not delete temp file");
		}

		// Remove from reserved files.
		let reserved_files:&mut Vec<FileRef> = unsafe { RESERVED_FILES.get_mut().unwrap() };
		if let Some(index) = reserved_files.iter().position(|entry| entry == &self.0) {
			reserved_files.remove(index);

			// If no reserved files and temp_file_dir is empty, delete dir.
			if reserved_files.is_empty() && TEMP_FILE_DIR.scanner().include_files().include_dirs().recurse().count() == 0 {
				let _ = TEMP_FILE_DIR.delete();
			}
		}
	}
}