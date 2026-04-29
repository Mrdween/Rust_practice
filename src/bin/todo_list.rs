#[derive(Debug)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
}

struct TodoList {
    tasks: Vec<Task>,
    next_id: u32,
}


impl TodoList {
    // 1. Создать новый список
    fn new() -> Self {
        TodoList {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    // 2. Добавить задачу (принимает String, возвращает id созданной задачи)
    fn add_task(&mut self, title: String) -> u32 {
        let id = self.next_id;
        self.next_id += 1;

        let task = Task {
            id,
            title,
            completed: false,
        };
        self.tasks.push(task);
        id

    }

    // 3. Найти задачу по id (вернуть ссылку, не забирая владение)
    fn find_task(&self, id: u32) -> Option<&Task> {
        // TODO: перебрать tasks, вернуть Some(task) если id совпадает
        for task in &self.tasks {
            if task.id == id {
                return Some(task);
            }
        }
        None
    }

    // 4. Найти изменяемую ссылку на задачу по id
    fn find_task_mut(&mut self, id: u32) -> Option<&mut Task> {
        // TODO: перебрать tasks, вернуть Some(&mut task) если id совпадает
        for task in &mut self.tasks {
            if task.id == id {
                return Some(task);
            }
        }
        None
    }

    // 5. Отметить задачу как выполненную
    fn complete_task(&mut self, id: u32) -> bool {
        // TODO: найти задачу по id (через find_task_mut), изменить completed на true
        // вернуть true если нашли, false если нет
        let task = self.find_task_mut(id);
        match task {
            Some(task) => {
                task.completed = true;
                true
            },
            None => false,
        }
    }

    // 6. Удалить задачу (забрать владение, вернуть удалённую задачу)
    fn remove_task(&mut self, id: u32) -> Option<Task> {
        // TODO: найти позицию задачи, удалить через remove()
        for i in 0..self.tasks.len() {
            if self.tasks[i].id == id {
                let removed = self.tasks.remove(i);
                return Some(removed);
            }
        }
        None
    }

    // 7. Получить все незавершённые задачи (вернуть новый Vec со ссылками)
    fn get_pending_tasks(&self) -> Vec<&Task> {
        let mut uncompleteds = Vec::new();
        for i in 0..self.tasks.len() {
            if self.tasks[i].completed == false {
                uncompleteds.push(&self.tasks[i]);
            }
        }
        uncompleteds
    }

    // 8. Подсчитать статистику (вернуть кортеж (всего, выполнено, осталось))
    fn stats(&self) -> (usize, usize, usize) {
        // TODO
        let total = self.tasks.len();
        let completed: usize = {
            let mut complete: u32 = 0;
            for task in &self.tasks {
                if task.completed == true {
                    complete += 1;
                } else {
                    continue;
                }
            }
            complete as usize
        };
        let stayed: usize = {
            let mut uncomplete: i32 = 0;
            for task in &self.tasks {
                if !task.completed {
                    uncomplete += 1;
                } else {
                    continue;
                }
            }
            uncomplete as usize
        };
        return (total, completed, stayed);
    }

    // 9. Забрать владение и показать все задачи (self)
    fn show_and_destroy(self) {
        dbg!(self.tasks);
    }
}

fn main() {
    let mut todo = TodoList::new();

    // Добавляем задачи
    let id1 = todo.add_task("Купить хлеб".to_string());
    let id2 = todo.add_task("Сдать проект".to_string());
    let id3 = todo.add_task("Позвонить маме".to_string());

    // Выполняем одну задачу
    todo.complete_task(id2);

    // Выводим незавершённые
    println!("Незавершённые задачи:");
    for task in todo.get_pending_tasks() {
        println!("  {}: {}", task.id, task.title);
    }

    // Статистика
    let (total, done, pending) = todo.stats();
    println!("Статистика: всего={}, выполнено={}, осталось={}", total, done, pending);

    // Удаляем задачу
    if let Some(removed) = todo.remove_task(id1) {
        println!("Удалена: {}", removed.title);
    }

    // Ищем несуществующую задачу
    match todo.find_task(999) {
        Some(t) => println!("Нашли: {}", t.title),
        None => println!("Задача 999 не найдена"),
    }

    // Показываем и уничтожаем
    todo.show_and_destroy();
    // После этого todo нельзя использовать
}
