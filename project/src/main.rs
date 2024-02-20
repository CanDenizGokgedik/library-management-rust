
struct Book {
    title: String,
    author: String,
    page_count: u32,
}

struct Magazine {
    title: String,
    issue: u32,
    topic: String,
}


enum Publication {
    Book(Book),
    Magazine(Magazine),
}


fn print_publication_info(publ: &Publication) {
    match publ {
        Publication::Book(book) => {
            println!("Book: {} author: {}, {} page", book.title, book.author, book.page_count);
        },
        Publication::Magazine(magazine) => {
            println!("Magazine: {} - Issue: {}, Topic: {}", magazine.title, magazine.issue, magazine.topic);
        },
    }
}

fn main() {
    let book = Book {
        title: String::from("Dune"),
        author: String::from("Frank Herbert"),
        page_count: 512,
    };

    let magazine = Magazine {
        title: String::from("Bitcoin Whitepaper"),
        issue: 1,
        topic: String::from("Proof of Work"),
    };

   
    let publications = vec![
        Publication::Book(book),
        Publication::Magazine(magazine),
    ];

    
    for publ in &publications {
        print_publication_info(publ);
    }
}
