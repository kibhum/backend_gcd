use std::time;
use tokio;

async fn print_odd() {
    let total = 10;
    let mut count = 0;
    let mut odd = 1;
    let three_seconds = time::Duration::from_millis(3000);
    while count < total {
        println!("Odd value: {odd}");
        odd += 2;
        count += 1;
        tokio::time::sleep(three_seconds).await;
    }
}

async fn print_even() {
    let total = 10;
    let mut count = 0;
    let mut even = 0;
    let three_seconds = time::Duration::from_millis(3000);
    while count < total {
        println!("Even value: {even}");
        even += 2;
        count += 1;
        tokio::time::sleep(three_seconds).await;
    }
}

fn func_with_closure<G>(f: G)
where
    G: FnOnce(&str),
{
    f("hello world");
}

#[tokio::main]
async fn main() {
    let odd = print_odd();
    let even = print_even();
    println!("Begin counting...");
    // This waits until both functions are executed
    tokio::join!(even, odd);
    println!("This is the end");
    let s = "The content of x is: ";
    let print_x_closure = |x: &str| {
        println!("{s} {x}");
    };
    func_with_closure(print_x_closure);
}
