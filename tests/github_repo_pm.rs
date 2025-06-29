// tests/github_url_tests.rs
use easygit::utils::valid_repo::is_valid_repo; // Usa el nombre del paquete (nombre en Cargo.toml)

#[test]
fn test_valid_https() {
    assert!(is_valid_repo("https://github.com/user/repo.git"));
    assert!(is_valid_repo("https://github.com/user-name/repo_name.git"));
}

#[test]
fn test_valid_ssh() {
    assert!(is_valid_repo("git@github.com:user/repo.git"));
    assert!(is_valid_repo("git@github.com:user.name/repo-name.git"));
}

#[test]
fn test_invalid_urls() {
    assert!(!is_valid_repo("https://github.com/user/repo"));      // falta .git
    assert!(!is_valid_repo("git@github.com:user/repo"));         // falta .git
    assert!(!is_valid_repo("https://gitlab.com/user/repo.git")); // dominio incorrecto
}
