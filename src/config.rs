use crate::session::TmuxSession;
use crate::session::TmuxWindow;

pub const TMUX: &str = "tmux";
pub const NEW_SESSION: &str = "new-session";
pub const ATTACH_SESSION: &str = "attach-session";
pub const NEW_WINDOW: &str = "new-window";
pub const BASE_PATH: &str = "/media/rolfie/ssd2/projects";
pub const BASE_PATH_COURS: &str = "/media/rolfie/ssd2/projects/cours";

const MINI_GAME: &str = "mini_games";
const STRATEGO: &str = "stratego";
const CINEMATOR: &str = "cinemator";
const QLEARNING: &str = "qlearning";
const AOC: &str = "aoc";
const RAY_TRACING: &str = "ray_tracing";
const DOT_FILES: &str = "dotfiles";
const TMUX_CREATOR: &str = "tmux-creator";

pub fn get_all_session() -> Vec<TmuxSession> {
    vec![
        TmuxSession::new(
            String::from(MINI_GAME),
            format!("{}/{}", BASE_PATH, MINI_GAME),
            Some(String::from("nvim")),
            String::from("vim"),
            vec![TmuxWindow::new(
                String::from("rust"),
                String::from(MINI_GAME),
                format!("{}/{}", BASE_PATH, MINI_GAME),
            )],
        ),
        TmuxSession::new(
            String::from(STRATEGO),
            format!("{}/{}", BASE_PATH_COURS, STRATEGO),
            Some(String::from("nvim")),
            String::from("vim"),
            vec![TmuxWindow::new(
                String::from("rust"),
                String::from(STRATEGO),
                format!("{}/{}", BASE_PATH_COURS, STRATEGO),
            )],
        ),
        TmuxSession::new(
            String::from(CINEMATOR),
            format!("{}/{}", BASE_PATH_COURS, CINEMATOR),
            Some(String::from("nvim")),
            String::from("vim"),
            vec![
                TmuxWindow::new(
                    String::from("server"),
                    String::from(CINEMATOR),
                    format!("{}/{}/server", BASE_PATH_COURS, CINEMATOR),
                ),
                TmuxWindow::new(
                    String::from("client"),
                    String::from(CINEMATOR),
                    format!("{}/{}/client", BASE_PATH_COURS, CINEMATOR),
                ),
            ],
        ),
        TmuxSession::new(
            String::from(QLEARNING),
            format!("{}/{}", BASE_PATH_COURS, QLEARNING),
            Some(String::from("nvim")),
            String::from("vim"),
            vec![TmuxWindow::new(
                String::from("python"),
                String::from(QLEARNING),
                format!("{}/{}", BASE_PATH_COURS, QLEARNING),
            )],
        ),
        TmuxSession::new(
            String::from(AOC),
            format!("{}/{}", BASE_PATH_COURS, AOC),
            Some(String::from("nvim")),
            String::from("vim"),
            vec![TmuxWindow::new(
                String::from("term"),
                String::from(AOC),
                format!("{}/{}", BASE_PATH_COURS, AOC),
            )],
        ),
        TmuxSession::new(
            String::from(RAY_TRACING),
            format!("{}/init-search/search/LR", BASE_PATH_COURS),
            Some(String::from("nvim")),
            String::from("vim"),
            vec![TmuxWindow::new(
                String::from("cpp"),
                String::from(RAY_TRACING),
                format!("{}/init-search/search/LR", BASE_PATH_COURS),
            )],
        ),
        TmuxSession::new(
            String::from(DOT_FILES),
            format!("/home/rolfie/Documents/{}", DOT_FILES),
            Some(String::from("nvim")),
            String::from("vim"),
            vec![],
        ),
        TmuxSession::new(
            String::from(TMUX_CREATOR),
            format!("{}/{}", BASE_PATH, TMUX_CREATOR),
            Some(String::from("nvim")),
            String::from("vim"),
            vec![TmuxWindow::new(
                String::from("rust"),
                String::from(TMUX_CREATOR),
                format!("{}/{}", BASE_PATH, TMUX_CREATOR),
            )],
        ),
    ]
}
