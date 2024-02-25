use std::io;
use chrono::{NaiveDate, Utc};

struct Task {
    id: u32,
    title: String,
    date: NaiveDate,
    done: bool,
}

struct TaskList {
    tasks: Vec<Task>,
}

impl TaskList {
    fn add(&mut self, title: String) {
        let task: Task = Task {
            id: (&self.tasks.len() + 1) as u32,
            title: title,
            date: Utc::now().date_naive(),
            done: false,
        };
        &self.tasks.push(task);
    }
}

fn main() {
    let mut task_list = TaskList {
        tasks: vec![]
    };

    clear();
    show_title();
    show_menu();
    get_option(&mut task_list);
}

fn clear() {
    print!("{}[2J{}[1;1H", 27 as char, 27 as char);
}

fn show_title() {
    println!("Lista de Tarefas");
    println!("----------------");
}

fn show_menu() {
    println!("Selecione uma opção:");
    println!("1 - Criar tarefa");
    println!("2 - Editar tarefa");
    println!("3 - Excluir tarefa");
    println!("4 - Concluir tarefa");
}

fn get_option(task_list: &mut TaskList) {
    loop {
        let mut option: String = String::new();
        io::stdin().read_line(&mut option).expect("Informe um número");

        let number = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        redirect(task_list, number);
    }
}

fn redirect(task_list: &mut TaskList, option: u8) {
    match option {
        1 => create_task(task_list),
        2 => println!("opção 2"),
        3 => println!("opção 3"),
        4 => println!("opção 4"),
        _ => println!("Opção inválida"),
    }
}

fn create_task(task_list: &mut TaskList) {
    clear();
    println!("Informe o nome da tarefa:");
    let mut task: String = String::new();
    io::stdin().read_line(&mut task).expect("Nome inválido");
    task_list.add(task);
    list_tasks(task_list);
}

fn remove_task(task_list: &mut TaskList) {
    clear();
    println!("Informe o ID da tarefa a ser removida:");
    loop {
        let mut id: String = String::new();
        io::stdin().read_line(&mut id).expect("Informe o ID de uma tarefa");
        let id: u32 = match id {
            Ok(num) => num,
            Err(_) => continue,
        };
    }
    
}

fn list_tasks(task_list: &mut TaskList) {
    clear();
    println!("Suas tarefas são:");
    for task in &task_list.tasks {
        println!(" - {}. {}", task.id, task.title)
    }

    show_menu();
}

#[cfg(test)]
mod test {
    use crate::TaskList;

    #[test]
    fn should_add_item_to_task_list() {
        let mut task_list = TaskList {
            tasks: vec![]
        };
        task_list.add("Nome da tarefa".parse().unwrap());

        assert_eq!(task_list.tasks.len(), 1);
    }
}