use std::collections::HashMap;
use std::env::set_current_dir;
use std::path::{Path};
use std::process::Command;

use core::errors::{Error, ErrorKind, Fallible, Result};
use core::utils::current_dir;
use vcs::{Repo, RepoInfo};

#[derive(Clone, Debug)]
pub struct Git{
    pub path: String,
    pub url: String,
    pub remotes: HashMap<String, String>,
}

impl Repo for Git {
    fn from_info(repo_info: RepoInfo) -> Self where Self: Sized {
        Git {
            path: repo_info.path,
            url: repo_info.url,
            remotes: repo_info.remotes,
        }
    }

    fn clone_repo(&self) {
        // Store current location
        let current_dir_string = current_dir().unwrap_or_fail();
        let current_dir = Path::new(&current_dir_string).to_path_buf();
        let repo_dir = current_dir.join(self.path.clone());
        let repo_name = repo_dir.file_name().unwrap();
        let parent_dir = repo_dir.parent().unwrap();

        let output = Command::new("mkdir")
                         .arg("-p")
                         .arg(parent_dir)
                         .output()
                         .unwrap();

        println!("{:#?}", output.stdout);
        println!("{:#?}", output.stderr);

        let output = Command::new("git")
                         .arg("clone")
                         .arg(self.url.clone())
                         .arg(repo_name)
                         .current_dir(parent_dir)
                         .output()
                         .unwrap();

        println!("{:#?}", output.stdout);
        println!("{:#?}", output.stderr);
    }

    fn kind(&self) -> String {
        "Git".to_string()
    }

    fn path(&self) -> String {
        self.path.clone()
    }

    fn remotes(&self) -> HashMap<String, String> {
        self.remotes.clone()
    }
}
