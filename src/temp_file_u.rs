#[cfg(test)]
mod tests {
	use file_ref::FileRef;
	use crate::TempFile;



	#[test]
	fn test_temp_file() {

		// Temp file should not exist on definition.
		let temp_file:TempFile = TempFile::new(None);
		assert!(!temp_file.0.exists(), "Temp file should not exist on definition.");

		// Temp file should be deleted on drop.
		temp_file.0.create().unwrap();
		assert!(temp_file.0.exists(), "Temp file should exist after create.");
		let temp_file_inner:FileRef = temp_file.0.clone();
		drop(temp_file);
		assert!(!temp_file_inner.exists(), "Temp file should not exist after drop.");
	}

	#[test]
	fn test_temp_files_can_be_created_async() {

		// Temp file should be able to be created. If it cannot be, it means it clashes with the other unit tests.
		let _temp_file:TempFile = TempFile::new(None);
	}

	#[test]
	fn test_temp_file_extension() {
		assert!(TempFile::new(Some("txt")).0.ends_with("txt"), "Temp file does not have correct extension.");
		assert!(TempFile::new(Some("png")).0.ends_with("png"), "Temp file does not have correct extension.");
	}
}