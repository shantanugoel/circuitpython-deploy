use circuitpython_deploy::ignore::IgnoreFilter;
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    let test_project = Path::new("test_project");
    
    if !test_project.exists() {
        eprintln!("test_project directory not found");
        return;
    }
    
    let filter = match IgnoreFilter::new(test_project) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to create ignore filter: {}", e);
            return;
        }
    };
    
    println!("Testing .cpdignore functionality in test_project/");
    println!("Files that would be included:");
    
    for entry in WalkDir::new(test_project)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        let relative_path = path.strip_prefix(test_project).unwrap();
        
        let is_included = filter.should_include(path);
        
        if is_included {
            println!("  ✓ {}", relative_path.display());
        } else {
            println!("  ✗ {} (ignored)", relative_path.display());
        }
        

    }
}
