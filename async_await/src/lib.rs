mod async_await {
    use futures::Future;

    async fn foo() -> u8 { 5 }

    fn bar() -> impl Future<Output = u8> {
        async {
            let x: u8 = foo().await;
            x + 5
        }
    }
}

mod lifetimes {
    use futures::Future;

    async fn foo(x: &u8) -> u8 { *x }

    fn foo_expanded<'a>(x: &'a u8) -> impl Future<Output = u8> + 'a {
        async move { *x }
    }


    async fn borrow_x(x: &u8) -> u8 { *x }

    // fn bad() -> impl Future<Output = u8> {
    //     let x = 5;
    //     borrow_x(&x)
    // }

    fn good() -> impl Future<Output = u8> {
        async {
            let x = 5;
            borrow_x(&x).await
        }
    }
}

mod movers {
    use futures::Future;

    async fn blocks() {
        let my_string = "foo".to_string();

        let future_one = async {
            println!("{:?}", my_string);
        };

        let future_two = async {
            println!("{:?}", my_string);
        };

        let ((), ()) = futures::join!(future_one, future_two);
    }

    fn move_block() -> impl Future<Output = ()> {
        let my_string = "foo".to_string();
        async move {
            println!("{:?}", my_string);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
