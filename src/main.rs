use std::fs;
use std::io::{self, Error, ErrorKind};

type Task = Box<dyn FnOnce() -> Result<(), Error>>;
type Rollback = Box<dyn FnOnce() -> Result<(), Error>>;

struct Transaction {
    execute_fn: Option<Task>,
    rollback_fn: Option<Rollback>,
}

impl Transaction {
    fn new<E, R>(execute_fn: E, rollback_fn: R) -> Self
    where
        E: Fn() -> Result<(), Error> + 'static,
        R: Fn() -> Result<(), Error> + 'static,
    {
        Transaction {
            execute_fn: Some(Box::new(execute_fn)),
            rollback_fn: Some(Box::new(rollback_fn)),
        }
    }

    fn execute_fn(&mut self) -> Result<(), Error> {
        //(self.execute_fn)()
        let execute_fn = self.execute_fn.take().expect("No execute_fn found");
        execute_fn()
    }

    fn rollback_fn(&mut self) -> Result<(), Error> {
        //(self.rollback_fn)()
        let rollback_fn = self.rollback_fn.take().expect("No rollback_fn found");
        rollback_fn()
    }
}

fn main() {
    let mut tasks: Vec<Transaction> = vec![
        Transaction::new(
            Box::new(|| copy_file_task()),
            Box::new(|| copy_file_rollback()),
        ),
        Transaction::new(
            Box::new(|| create_directory_task()),
            Box::new(|| create_directory_rollback()),
        ),
    ];

    let mut rollback_required = false;

    // Execute tasks
    for task in &mut tasks {
        match task.execute_fn() {
            Ok(()) => {}
            Err(err) => {
                println!("Task execution failed: {}", err);
                rollback_required = true;
                break;
            }
        }
    }

    // Rollback tasks if necessary
    if rollback_required {
        for task in tasks.iter_mut().rev() {
            match task.rollback_fn() {
                Ok(()) => {}
                Err(err) => println!("Rollback failed: {}", err),
            }
        }
    }
}

fn copy_file_task() -> Result<(), Error> {
    println!("Copying file");
    // Perform file copying logic here
    Ok(())
}

fn copy_file_rollback() -> Result<(), Error> {
    println!("Rolling back file copy");
    // Perform rollback logic here
    Ok(())
}

fn create_directory_task() -> Result<(), Error> {
    println!("Creating directory");
    // Perform directory creation logic here
    Ok(())
}

fn create_directory_rollback() -> Result<(), Error> {
    println!("Rolling back directory creation");
    // Perform rollback logic here
    Ok(())
}
