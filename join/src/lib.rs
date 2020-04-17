use futures::join;

struct Book;
struct Music;

async fn get_book() -> Book { Book }
async fn get_music() -> Music { Music }


async fn get_book_and_music() -> (Book, Music) {
    let book_fut = get_book();
    let music_fut = get_music();
    join!(book_fut, music_fut)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
