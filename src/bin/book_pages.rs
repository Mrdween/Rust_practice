struct Book {
    title: String,
    pages: u32,
}

// TODO: написать функцию, которая находит книгу с максимальным количеством страниц
// Вернуть Option<&Book>
fn find_thickest(books: &[Book]) -> Option<&Book> {
    if books.is_empty() {
        return None;
    }

    let mut thickets = &books[0];
    for book in books {
        if book.pages > thickets.pages {
            thickets = book;
        }
    }
    Some(thickets)
}

fn main() {
    let books = vec![
        Book { title: "А".to_string(), pages: 100 },
        Book { title: "Б".to_string(), pages: 300 },
        Book { title: "В".to_string(), pages: 200 },
    ];

    if let Some(book) = find_thickest(&books) {
        println!("{}", book.title); // "Б"
    }
}
