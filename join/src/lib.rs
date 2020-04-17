use futures::join;
use futures::try_join;
use futures::future::{TryFutureExt, try_join};

struct Book;
struct Music;

async fn get_book() -> Book { Book }
async fn get_music() -> Music { Music }

async fn get_book_and_music() -> (Book, Music) {
    let book_fut = get_book();
    let music_fut = get_music();
    join!(book_fut, music_fut)
}

async fn try_join_get_book() -> Result<Book, ()> { Ok(Book) }
async fn try_join_get_music() -> Result<Music, ()> { Ok(Music) }

async fn try_join_get_book_and_music() -> Result<(Book, Music), String> {
    let book_fut = try_join_get_book().map_err(|()| "Unable to get book".to_string());
    let music_fut = try_join_get_music().map_err(|()| "Unable to get music".to_string());
    try_join!(book_fut, music_fut)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
