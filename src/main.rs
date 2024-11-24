use std::io;

// リストの構造体
#[derive(Debug)]
struct List {
    name: String,
    tasks: Vec<Task>,
}

// タスクの構造体
#[derive(Debug)]
struct Task {
    name: String,
    done: bool,
}

fn main() -> io::Result<()> {
    println!("Todo List!");

    // サンプル
    let mut lists: Vec<List> = Vec::new();

    let tasks: Vec<Task> = Vec::new();
    let mut list: List = List {
        name: String::from("OS"),
        tasks: tasks,
    };

    let task = Task {
        name: String::from("OS3.3"),
        done: true,
    };

    list.tasks.push(task);
    lists.push(list);

    println!("{:?}", lists);

    let mut home_flag = true;

    loop {
        // リストの操作
        if home_flag {
            // 入力待ち状態
            println!(
                "\nEnter a command: 
    show => show lists
    create => create list
    change => change task mode
    exit => exit program\n"
            );

            // 入力受け取り
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to readline");

            // 入力の改行などを取り除く
            let command = input.trim();

            // 入力の内容に応じた処理
            match command {
                "show" => {
                    println!();
                    let mut num: usize = 1;
                    for list in &lists {
                        println!("{}: {}", num, list.name);
                        num += 1;
                    }
                }
                "create" => {}
                "change" => {
                    home_flag = false;
                    // 入力待ち状態
                    println!("which list would you choose?");

                    // 入力受け取り
                    let mut input = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("failed to readline");

                    //　入力の改行などを取り除く
                    let index = input.trim().parse::<usize>().unwrap();
                }
                "exit" => break,
                "" => continue,
                _ => println!("command not found"),
            }

        // タスクの操作
        } else {
            // 入力待ち状態
            println!(
                "\nEnter a command: 
    show => show tasks
    create => create task
    change => change list mode
    exit => exit program\n"
            );

            // 入力受け取り
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to readline");

            // 入力の改行などを取り除く
            let command = input.trim();

            // 入力の内容に応じた処理
            match command {
                "show" => {
                    // // リストの取得
                    // let mut list = lists.get(index - 1).unwrap();

                    // let mut num: usize = 1;
                    // for task in &list.tasks {
                    //     println!("{}: {:?}", num, task);
                    //     num += 1;
                    // }
                }
                "create" => {}
                "change" => home_flag = true,
                "" => continue,
                _ => println!("command not found"),
            }
        }
    }

    Ok(())
}
