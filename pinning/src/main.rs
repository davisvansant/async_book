// use pin_utils::pin_mut;
use futures::Future;
use futures::pin_mut;

fn execute_unpin_future(x: impl Future<Output = ()> + Unpin) {}



fn main() {
    println!("Hello, world!");

    let fut = async {

    };

    // execute_unpin_future(fut);

    let futurarma = async {};
    let futfut = Box::pin(futurarma);
    execute_unpin_future(futfut);
    let futfutfut = async {

    };
    pin_mut!(futfutfut);
    execute_unpin_future(futfutfut);
}
