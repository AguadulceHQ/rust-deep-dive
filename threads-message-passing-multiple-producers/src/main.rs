use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let projects = vec![
            String::from("Project 1 from BD 1"),
            String::from("Project 2 from BD 1"),
            String::from("Project 3 from BD 1"),
        ];

        for project in projects {
            tx1.send(project).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });

    // 2nd thread

    thread::spawn(move || {
        let projects = vec![
            String::from("Project 4 from BD 2"),
            String::from("Project 5 from BD 2"),
            String::from("Project 6 from BD 2"),
        ];

        for project in projects {
            tx.send(project).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });

    for project in rx {
        println!("New project added to pipeline: {}", project)
    }
}
