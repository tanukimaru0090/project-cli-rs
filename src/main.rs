/****main.rs****/
/****project-cli****/
/****in Console Application****/
mod example_codes;
use clap::{Parser, Subcommand, ValueEnum};
use example_codes::{
    cpp_example_code::*, haxe_example_code::*, java_example_code::*, python_example_code::*,
    rust_example_code::*,
};
use std::fs::*;
use std::io::Write;
use std::process::{Command, Output};
// newコマンド
#[derive(Debug, Parser)]
struct New {
    #[arg(short,long,required = true,default_value="cpp",value_name="LanguageName")]
    lang: LanguageType,
    #[arg(value_name = "ProjectName")]
    name: String,
    #[arg(
        short,
        long,
        default_value = "hello_world",
        required = false,
        value_name = "ExampleName"
    )]
    example: String,
    #[arg(
        short,
        long,
        required = false,
        default_value = "",
        allow_hyphen_values(true),
        value_delimiter = ','
    )]
    args: Vec<String>,
}
// delコマンド
#[derive(Debug, Parser)]
struct Del {
    #[arg(value_name = "ProjectName")]
    name: String,
    #[arg(
        short,
        long,
        required = false,
        default_value = "",
        allow_hyphen_values(true),
        value_delimiter = ','
    )]
    args: Vec<String>,
}
// サブコマンド (new,del)
#[derive(Debug, Subcommand)]
enum MainSubCommand {
    #[command(long_about = "Create a new project\nwith the specified language and name :)\n")]
    New(New),
    #[command(long_about = "delete a project\nwith the specified projectname :*\n")]
    Del(Del),
}
// メインコマンド
#[derive(Debug, Parser)]
struct MainCommand {
    #[command(subcommand)]
    command: MainSubCommand,
}
// 言語の種類
#[derive(Debug, Clone, clap::ValueEnum)]
enum LanguageType {
    Cpp,
    Rust,
    Csharp,
    Java,
    Python,
    Haxe,
}

// 言語に応じたサンプルコードの種類
enum CppExampleType {}
enum RustExampleType {}
enum JavaExampleType {}
// 外部コマンドの結果を表示する
fn print_command_output(output: &Output) {
    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));
}
// 言語に共通のファイルを作成する (主流のパッケージマネージャーがない場合)
fn create_default_files(name: &str) {}
// 言語に共通のディレクトリを作成する  (主流のパッケージマネージャーがない場合)

fn create_default_directory(name: &str) -> Result<(), String> {
    let proj_path = name.to_string() + "/";
    let default_dir: Vec<&str> = vec!["src", "test", "doc", "scripts", "config"];

    for dir in &default_dir {
        if let Err(e) = create_dir_all(proj_path.clone() + dir) {
            return Err(format!("ディレクトリの作成に失敗しました: {}", e));
        }
    }

    Ok(())
}

// 言語に応じてプロジェクトディレクトリを作成する
fn create_project_directory(new: &New, out_print: bool) {
    match new.lang {
        LanguageType::Python => {
            let proj_name = new.name.clone();
            let proj_path = new.name.to_string() + "/";
            // デフォルトの共通ディレクトリの作成
            if let Err(e) = create_default_directory(&proj_name) {
            } else {
                // サンプルソースの作成
                let mut file = match File::create(proj_path.clone() + "src/main.py") {
                    Ok(file) => file,
                    Err(e) => {
                        println!("ファイルの作成に失敗しました {}", e);
                        return;
                    }
                };
                // サンプルソースの書き込み
                if let Err(e) = write!(file, "{}", PYTHON_EXAMPLE_CODE_2) {
                    println!("ファイルの書き込みに失敗しました {}", e);
                    return;
                }
            }
        }
        LanguageType::Cpp => {
            let proj_name = new.name.clone();
            let proj_path = proj_name.clone() + "/";
            let cmake_conf_path = proj_path.clone() + "CMakeLists.txt";
            // ソースディレクトリの作成
            if let Err(e) = create_dir_all(proj_path.clone() + "src") {
                println!("ディレクトリの作成に失敗しました {}", e);
                return;
            } else {
                // サンプルソースの作成
                let mut file = match File::create(proj_path.clone() + "src/main.cc") {
                    Ok(file) => file,
                    Err(e) => {
                        println!("ファイルの作成に失敗しました {}", e);
                        return;
                    }
                };
                // サンプルソースの書き込み
                if let Err(e) = write!(file, "{}", CPP_EXAMPLE_CODE_1) {
                    println!("ファイルの書き込みに失敗しました {}", e);
                    return;
                }
            }
            // ビルドディレクトリの作成
            if let Err(e) = create_dir_all(proj_path.clone() + "build") {
                println!("ディレクトリの作成に失敗しました {}", e);
                return;
            }
            let cmake_conf_text = format!(
                "project(\"{}\")\nadd_executable({} src/main.cc)",
                proj_name.clone(),
                proj_name.clone()
            );
            // CMakeLists.txtの作成
            let mut file = match File::create(&cmake_conf_path) {
                Ok(file) => file,
                Err(e) => {
                    println!("ファイルの作成に失敗しました {}", e);
                    return;
                }
            };

            if let Err(e) = write!(file, "{}{}", CMAKE_DEFAULT_CODE, cmake_conf_text) {
                println!("ファイルの書き込みに失敗しました {}", e);
                return;
            }
        }
        LanguageType::Haxe => {
            let proj_name = new.name.clone();
            let proj_path = proj_name + "/";
            let build_conf_path = proj_path.clone() + "build.hxml";
            // ソースディレクトリの作成
            if let Err(e) = create_dir_all(proj_path.clone() + "src") {
                println!("ディレクトリの作成に失敗しました {}", e);
                return;
            } else {
                // サンプルソースの作成
                let mut file = match File::create(proj_path.clone() + "src/Main.hx") {
                    Ok(file) => file,
                    Err(e) => {
                        println!("ファイルの作成に失敗しました {}", e);
                        return;
                    }
                };
                // サンプルソースの書き込み
                if let Err(e) = write!(file, "{}", HAXE_EXAMPLE_CODE_2) {
                    println!("ファイルの書き込みに失敗しました {}", e);
                    return;
                }
            }
            let _build_conf_text = format!("-cp src\n-main Main\n",);
            // build.hxmlの作成
            let mut file = match File::create(&build_conf_path) {
                Ok(file) => file,
                Err(e) => {
                    println!("ファイルの作成に失敗しました {}", e);
                    return;
                }
            };

            if let Err(e) = write!(file, "{}", HAXE_BUILD_DEFAULT_CODE) {
                println!("ファイルの書き込みに失敗しました {}", e);
                return;
            }
        }
        LanguageType::Csharp => {
            let mut cmd = Command::new("dotnet");
            let args = new.args.clone();

            if args.iter().all(|s| s.trim().is_empty()) {
                let out = cmd
                    .arg("new")
                    .arg("console")
                    .arg("-n")
                    .arg(&new.name)
                    .output()
                    .expect("dotnet command failed to run");
                if out_print {
                    print_command_output(&out);
                }
            } else {
                let out = cmd
                    .arg("new")
                    .arg("console")
                    .arg("-n")
                    .arg(&new.name)
                    .args(&args)
                    .output()
                    .expect("dotnet command failed to run");
                if out_print {
                    print_command_output(&out);
                }
            }
        }
        LanguageType::Rust => {
            let mut cmd = Command::new("cargo");
            let args = new.args.clone();
            if args.iter().all(|s| s.trim().is_empty()) {
                let out = cmd
                    .arg("new")
                    .arg(&new.name)
                    .output()
                    .expect("cargo command failed to run");

                if out_print {
                    print_command_output(&out);
                }
            } else {
                let out = cmd
                    .arg("new")
                    .arg(&new.name)
                    .args(&args)
                    .output()
                    .expect("cargo command failed to run");
                if out_print {
                    print_command_output(&out);
                }
            }
        }
        LanguageType::Java => {}
        _ => {}
    }
}

// 指定のプロジェクトを削除する
fn delete_project_directory(del: &Del, out_print: bool) {
    let out = Command::new("rm")
        .arg("-r")
        .arg(&del.name)
        .output()
        .expect("rm command failed to run");
    if let Some(status) = out.status.code() {
        if status == 0 {
            println!("プロジェクトの削除に成功しました exit status:{}", status);
        } else {
            println!("プロジェクトの削除に失敗しました exit status:{}", status);
        }
    }
    if out_print {
        print_command_output(&out);
    }
}
// メイン関数
fn main() {
    // 引数のパース
    let main_args = MainCommand::parse();
    match main_args.command {
        MainSubCommand::New(new) => {
            create_project_directory(&new, true);
        }
        MainSubCommand::Del(del) => {
            delete_project_directory(&del, true);
        }
    }
}
