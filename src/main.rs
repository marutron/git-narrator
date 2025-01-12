use git2;
use git2::{Repository, Oid};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Получаем аргументы командной строки
    let args: Vec<String> = env::args().collect();

    let repo_path = "/home/marat/repo/git-narrator/.git";
    let merge_commit_sha = "72aa2a3923617f0154164908bab979336c0a81a7";

    process_merge_commit(repo_path, merge_commit_sha)?;

    Ok(())
}

fn process_merge_commit(repo_path: &str, merge_commit_sha: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Открываем репозиторий
    let repo = Repository::open(repo_path)?;

    // Получаем мерж-коммит по SHA
    let merge_commit_oid = Oid::from_str(merge_commit_sha)?;
    let merge_commit = repo.find_commit(merge_commit_oid)?;

    // Получаем родительские коммиты
    let mut parents = merge_commit.parents();

    if parents.len() < 1 {
        println!("No parent commits found for the merge commit.");
        return Ok(());
    }

    // Предполагаем, что первый родитель - это ветка master
    let master_parent = parents.next().unwrap();

    // Получаем все коммиты между родителем и мерж-коммитом
    let mut revwalk = repo.revwalk()?;
    revwalk.push(master_parent.id())?;
    revwalk.hide(merge_commit.id())?;

    println!("Commits merged into master:");

    for oid in revwalk {
        let oid = oid?;
        println!("{:?}", oid);
        let commit = repo.find_commit(oid)?;

        // Обработка коммитов по шаблону
        println!("Processing commit: {} - {}", commit.id(), commit.message().unwrap_or("No message"));
    }

    Ok(())
}
