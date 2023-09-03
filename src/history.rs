use git2::{Repository, Oid, BranchType};
use std::error::Error;
use std::path::Path;
use git2::RepositoryState::Clean;
use ignore::WalkBuilder;

use crate::{find_private_keys, main as checkerMain};

fn get_all_commits(repo: &Repository) -> Result<Vec<Oid>, Box<dyn Error>> {
    let mut revwalk = repo.revwalk()?;
    revwalk.push_glob("*")?; // get commits from all branches
    let mut commits = Vec::new();
    for id in revwalk {
        let id = id?;
        commits.push(id);
    }
    Ok(commits)
}

fn checkout_commit(repo: &Repository, id: &Oid) -> Result<(), Box<dyn Error>> {
    let commit = repo.find_commit(*id)?;
    let tree = commit.tree()?;
    let mut checkout_builder = git2::build::CheckoutBuilder::new();
    checkout_builder.force(); // force checkout (overwrite any changes)
    repo.checkout_tree(&tree.into_object(), Some(&mut checkout_builder))?;
    repo.set_head_detached(*id)?;
    Ok(())
}

pub fn check_history() {
    let repo = Repository::discover(".").expect("Couldn't open repository");
    let head = repo.head().expect("Couldn't find head").resolve().unwrap();
    let commit = repo.find_commit(head.target().unwrap()).unwrap();
    let initial_branch_name = head.name();
    /* Check if working directory is clean, abort otherwise */
    if !repo.state().eq(&Clean) {
        println!("Working directory not clean, aborting history check to avoid losing changes.");
        return;
    }

    let mut private_keys: Vec<(std::path::PathBuf, usize)> = vec![];

    let commits = get_all_commits(&repo).unwrap();
    for id in &commits {
        checkout_commit(&repo, id).unwrap();
        let mut builder = WalkBuilder::new("./");
        builder.standard_filters(false)
            .hidden(false)
            .parents(false)
            .git_ignore(true);

        let path = Path::new(".keycheckignore");
        if path.is_file() {
            builder.add_ignore(path);
        }

        private_keys.append(&mut find_private_keys(builder));
    }

    /* Checkout head after running through all commits */
    repo.checkout_tree(
        &commit.tree().unwrap().into_object(),
        None,
    ).expect("Couldn't checkout initial commit");

    repo.set_head(&("refs/heads".to_owned() + initial_branch_name.unwrap())).expect("Couldn't checkout initial commit");

    if !private_keys.is_empty() {
        println!("Warning: private keys found in the following files:");
        for (path, line_number) in private_keys {
            println!("{}:{}", path.display(), line_number);
        }

        std::process::exit(1);
    }
}
