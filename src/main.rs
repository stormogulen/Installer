//use std::error::Error;
//use std::fs;
use std::io::{self, Error, ErrorKind};

type Task = Box<dyn FnOnce() -> Result<(), Error>>;
type Rollback = Box<dyn FnOnce() -> Result<(), Error>>;
type Script = Box<dyn FnOnce() -> Result<(), Error>>;

struct Transaction {
    execute_fn: Option<Task>,
    rollback_fn: Option<Rollback>,
    script_fn: Option<Script>,
}

impl Transaction {
    fn new<E, R, S>(execute_fn: E, rollback_fn: R, script_fn: S) -> Self
    where
        E: Fn() -> Result<(), Error> + 'static,
        R: Fn() -> Result<(), Error> + 'static,
        S: FnOnce() -> Result<(), Error> + 'static,
    {
        Transaction {
            execute_fn: Some(Box::new(execute_fn)),
            rollback_fn: Some(Box::new(rollback_fn)),
            script_fn: Some(Box::new(script_fn)),
        }
    }

    // fn new<E, R>(execute_fn: E, rollback_fn: R) -> Self
    // where
    //     E: Fn() -> Result<(), Error> + 'static,
    //     R: Fn() -> Result<(), Error> + 'static,
    // {
    //     Transaction {
    //         execute_fn: Some(Box::new(execute_fn)),
    //         rollback_fn: Some(Box::new(rollback_fn)),
    //     }
    // }

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

    fn script_fn(&mut self) -> Result<(), Error> {
        let script_fn = self.script_fn.take().expect("No script_fn found");
        script_fn()
    }
}

fn main() {
    let mut tasks: Vec<Transaction> = vec![
        Transaction::new(
            Box::new(|| copy_file_task()),
            Box::new(|| copy_file_rollback()),
            Box::new(|| run_script_task()),
        ),
        Transaction::new(
            Box::new(|| create_directory_task()),
            Box::new(|| create_directory_rollback()),
            Box::new(|| create_run_script_task()),
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

        match task.script_fn() {
            Ok(()) => {}
            Err(err) => {
                println!("Run script failed: {}", err);
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

fn run_script_task() -> Result<(), Error> {
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

fn create_run_script_task() -> Result<(), Error> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_file_task() -> Result<(), Box<Error>> {
        // Test copying a file (replace with actual test logic)
        let result = copy_file_task();
        assert!(result.is_ok()); // Check if the task completed successfully
        Ok(())
    }

    #[test]
    fn test_copy_file_rollback() -> Result<(), Box<Error>> {
        // Test rolling back file copy (replace with actual test logic)
        let result = copy_file_rollback();
        assert!(result.is_ok()); // Check if the rollback completed successfully
        Ok(())
    }

    #[test]
    fn test_create_directory_task() -> Result<(), Box<Error>> {
        // Test creating a directory (replace with actual test logic)
        let result = create_directory_task();
        assert!(result.is_ok()); // Check if the task completed successfully
        Ok(())
    }

    #[test]
    fn test_create_directory_rollback() -> Result<(), Box<Error>> {
        // Test rolling back directory creation (replace with actual test logic)
        let result = create_directory_rollback();
        assert!(result.is_ok()); // Check if the rollback completed successfully
        Ok(())
    }

    #[test]
    fn test_run_script_task() -> Result<(), Box<Error>> {
        let result = run_script_task();
        assert!(result.is_ok()); // Check if the task completed successfully
        Ok(())
    }

    #[test]
    fn test_create_run_script_task() -> Result<(), Box<Error>> {
        let result = create_run_script_task();
        assert!(result.is_ok()); // Check if the task completed successfully
        Ok(())
    }
}
