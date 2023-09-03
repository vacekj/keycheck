use git2::{Repository, Oid};
use std::error::Error;
use git2::RepositoryState::Clean;

use crate::main as checkerMain;

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

fn main() {
    let repo = Repository::discover(".").unwrap();

    /* Check if working directory is clean, abort otherwise */
    if !repo.state().eq(&Clean) {
        println!("Working directory not clean, aborting history check to avoid losing changes.");
        return;
    }

    let commits = get_all_commits(&repo).unwrap();
    for id in &commits {
        checkout_commit(&repo, id).unwrap();
        checkerMain();
    }

    /* Checkout head after running through all commits, even if main finds a match */

}
