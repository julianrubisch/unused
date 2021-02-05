use ignore::{WalkBuilder, WalkState};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub struct CodebaseFiles {
    pub paths: Vec<PathBuf>,
}

impl CodebaseFiles {
    pub fn all() -> CodebaseFiles {
        let builder = WalkBuilder::new("./");

        let results = Arc::new(Mutex::new(vec![]));

        builder.build_parallel().run(|| {
            Box::new(|result| {
                if let Ok(entry) = result {
                    let mut results = results.lock().unwrap();
                    results.push(entry.path().to_path_buf());
                }

                WalkState::Continue
            })
        });

        let mut paths = results.lock().unwrap().to_vec();
        paths.sort();

        CodebaseFiles { paths }
    }
}
