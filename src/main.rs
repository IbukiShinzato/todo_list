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

    // let mut entries = fs::read_dir("list")?
    //     .map(|res| res.map(|e| e.path()))
    //     .collect::<Result<Vec<_>, io::Error>>()?;

    // entries.sort();

    // let mut lists: Vec<List> = Vec::new();

    // let tasks: Vec<Task> = Vec::new();
    // let mut list: List = List {
    //     name: String::from("OS"),
    //     tasks: tasks,
    // };

    // let task = Task {
    //     name: String::from("OS1.3"),
    //     done: true,
    // };

    // list.tasks.push(task);

    // lists.push(list);

    // println!("{:?}", lists);

    loop {
        // 入力待ち状態
        println!(
            "\nEnter a command: 
    stop => exit program
    s list => show list
    s task => show task in list
    c list => create list
    c task => create task in list"
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
            "stop" => break,
            _ => continue,
        }
    }

    Ok(())
}
