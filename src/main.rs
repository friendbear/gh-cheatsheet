use clap::Arg;
use clap::App;
use std::env;

const BASIC_COMMANDS_EN: &str = r#"
Basic Git Commands:
+-------------------------+------------------------------------------+
| Command                 | Description                              |
+-------------------------+------------------------------------------+
| clone <repo>            | Clone a repository into a new directory  |
| init                    | Create an empty Git repository           |
| add <file>              | Add file contents to the index           |
| commit -m "<message>"   | Record changes to the repository         |
| status                  | Show the working tree status             |
+-------------------------+------------------------------------------+
"#;

const BASIC_COMMANDS_JP: &str = r#"
基本的な Git コマンド:
+-------------------------+------------------------------------------+
| コマンド                | 説明                                     |
+-------------------------+------------------------------------------+
| clone <repo>            | リポジトリを新しいディレクトリにクローン |
| init                    | 空の Git リポジトリを作成                |
| add <file>              | ファイルの内容をインデックスに追加        |
| commit -m "<message>"   | リポジトリに変更を記録                   |
| status                  | 作業ツリーの状態を表示                   |
+-------------------------+------------------------------------------+
"#;

const MIDDLE_LEVEL_COMMANDS_EN: &str = r#"
Middle-level Git Commands:
+-------------------------+------------------------------------------+
| Command                 | Description                              |
+-------------------------+------------------------------------------+
| branch <branch-name>    | List, create, or delete branches         |
| checkout <branch-name>  | Switch branches or restore working tree  |
| merge <branch>          | Join two or more development histories   |
| log                     | Show commit logs                         |
| reset <commit>          | Reset current HEAD to the specified state|
+-------------------------+------------------------------------------+
"#;

const MIDDLE_LEVEL_COMMANDS_JP: &str = r#"
中級の Git コマンド:
+-------------------------+------------------------------------------+
| コマンド                | 説明                                     |
+-------------------------+------------------------------------------+
| branch <branch-name>    | ブランチの一覧表示、作成、削除           |
| checkout <branch-name>  | ブランチの切り替えまたは作業ツリーの復元  |
| merge <branch>          | 2つ以上の開発履歴を結合                 |
| log                     | コミットログを表示                       |
| reset <commit>          | 現在の HEAD を指定した状態にリセット     |
+-------------------------+------------------------------------------+
"#;

const ADVANCED_COMMANDS_EN: &str = r#"
Advanced Git Commands:
+-------------------------+------------------------------------------+
| Command                 | Description                              |
+-------------------------+------------------------------------------+
| rebase <branch>         | Reapply commits on top of another base   |
| cherry-pick <commit>    | Apply the changes introduced by some     |
| stash                   | Stash the changes in a dirty working dir |
| fetch                   | Download objects and refs from another   |
| pull                    | Fetch from and integrate with another    |
| push                    | Update remote refs along with associated |
+-------------------------+------------------------------------------+
"#;

const ADVANCED_COMMANDS_JP: &str = r#"
上級の Git コマンド:
+-------------------------+------------------------------------------+
| コマンド                | 説明                                     |
+-------------------------+------------------------------------------+
| rebase <branch>         | 別のベースの上にコミットを再適用        |
| cherry-pick <commit>    | 既存のコミットによって導入された変更を適用|
| stash                   | 汚れた作業ディレクトリの変更を一時保存  |
| fetch                   | 別のリポジトリからオブジェクトとリファレンスをダウンロード |
| pull                    | 別のリポジトリまたはローカルブランチからフェッチして統合 |
| push                    | リモートリファレンスと関連オブジェクトを更新 |
+-------------------------+------------------------------------------+
"#;

const MERGE_DETAILS_EN: &str = r#"
Git Merge Command:
+-------------------------+------------------------------------------+
| Command                 | Description                              |
+-------------------------+------------------------------------------+
| merge <branch>          | Join two or more development histories   |
+-------------------------+------------------------------------------+
| Suboption               | Description                              |
+-------------------------+------------------------------------------+
| --no-ff                 | Create a merge commit even when the      |
|                         | merge resolves as a fast-forward         |
| --squash                | Merge changes but do not commit them     |
| --strategy <strategy>   | Use the given merge strategy             |
+-------------------------+------------------------------------------+

Example:
    git merge feature-branch   Merge 'feature-branch' into the current branch
"#;

const MERGE_DETAILS_JP: &str = r#"
Git Merge コマンド:
+-------------------------+------------------------------------------+
| コマンド                | 説明                                     |
+-------------------------+------------------------------------------+
| merge <branch>          | 2つ以上の開発履歴を結合                 |
+-------------------------+------------------------------------------+
| サブオプション          | 説明                                     |
+-------------------------+------------------------------------------+
| --no-ff                 | マージがファストフォワードで解決する場合もマージコミットを作成 |
| --squash                | 変更をマージするがコミットはしない       |
| --strategy <strategy>   | 指定したマージ戦略を使用                |
+-------------------------+------------------------------------------+

例:
    git merge feature-branch   'feature-branch'を現在のブランチにマージ
"#;

const REBASE_DETAILS_EN: &str = r#"
Git Rebase Command:
+-------------------------+------------------------------------------+
| Command                 | Description                              |
+-------------------------+------------------------------------------+
| rebase <branch>         | Reapply commits on top of another base   |
+-------------------------+------------------------------------------+
| Suboption               | Description                              |
+-------------------------+------------------------------------------+
| --interactive           | Make a list of the commits which are about to be rebased |
| --onto <newbase>        | Transplant commits onto the given base   |
| --autosquash            | Automatically move commits that begin with squash! or fixup! |
+-------------------------+------------------------------------------+

Example:
    git rebase master      Reapply commits on top of the master branch
"#;

const REBASE_DETAILS_JP: &str = r#"
Git Rebase コマンド:
+-------------------------+------------------------------------------+
| コマンド                | 説明                                     |
+-------------------------+------------------------------------------+
| rebase <branch>         | 別のベースの上にコミットを再適用        |
+-------------------------+------------------------------------------+
| サブオプション          | 説明                                     |
+-------------------------+------------------------------------------+
| --interactive           | リベースしようとしているコミットのリストを作成 |
| --onto <newbase>        | 指定したベースにコミットを移植          |
| --autosquash            | squash! または fixup! で始まるコミットを自動的に移動 |
+-------------------------+------------------------------------------+

例:
    git rebase master      master ブランチの上にコミットを再適用
"#;

const CHERRY_PICK_DETAILS_EN: &str = r#"
Git Cherry-pick Command:
+-------------------------+------------------------------------------+
| Command                 | Description                              |
+-------------------------+------------------------------------------+
| cherry-pick <commit>    | Apply the changes introduced by some     |
|                         | existing commits                         |
+-------------------------+------------------------------------------+
| Suboption               | Description                              |
+-------------------------+------------------------------------------+
| --no-commit             | Apply the changes but do not commit them |
| --edit                  | Edit the commit message before committing|
| --signoff               | Add a Signed-off-by line at the end of the commit message |
+-------------------------+------------------------------------------+

Example:
    git cherry-pick abc123 Apply the changes introduced by commit abc123
"#;

const CHERRY_PICK_DETAILS_JP: &str = r#"
Git Cherry-pick コマンド:
+-------------------------+------------------------------------------+
| コマンド                | 説明                                     |
+-------------------------+------------------------------------------+
| cherry-pick <commit>    | 既存のコミットによって導入された変更を適用 |
+-------------------------+------------------------------------------+
| サブオプション          | 説明                                     |
+-------------------------+------------------------------------------+
| --no-commit             | 変更を適用するがコミットしない          |
| --edit                  | コミットメッセージを編集                |
| --signoff               | コミットメッセージの最後に Signed-off-by 行を追加 |
+-------------------------+------------------------------------------+

例:
    git cherry-pick abc123 コミット abc123 によって導入された変更を適用
"#;

fn print_help(language: &str) {
    match language {
        "jp" => {
            println!("{}", BASIC_COMMANDS_JP);
            println!("{}", MIDDLE_LEVEL_COMMANDS_JP);
            println!("{}", ADVANCED_COMMANDS_JP);
        }
        _ => {
            println!("{}", BASIC_COMMANDS_EN);
            println!("{}", MIDDLE_LEVEL_COMMANDS_EN);
            println!("{}", ADVANCED_COMMANDS_EN);
        }
    }
}

fn print_command_details(command: &str, language: &str) {
    match (command, language) {
        ("merge", "jp") => println!("{}", MERGE_DETAILS_JP),
        ("merge", _) => println!("{}", MERGE_DETAILS_EN),
        ("rebase", "jp") => println!("{}", REBASE_DETAILS_JP),
        ("rebase", _) => println!("{}", REBASE_DETAILS_EN),
        ("cherry-pick", "jp") => println!("{}", CHERRY_PICK_DETAILS_JP),
        ("cherry-pick", _) => println!("{}", CHERRY_PICK_DETAILS_EN),
        _ => eprintln!("No details available for the command: {}", command),
    }
}

fn main() {
    let matches = App::new("example")
    .arg(Arg::with_name("command")
        .long("command")
        .takes_value(false)
        .help("Specify command")
        .required(false))
    .get_matches();

    let language = env::var("LANG")
        .unwrap_or_else(|_| "en".to_string())
        .split('_')
        .next()
        .unwrap_or("en")
        .to_string();

    if let Some(command) = matches.value_of("command") {
        print_command_details(command, &language);
    } else {
        print_help(&language);
    }
}
