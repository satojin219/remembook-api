use kernel::model::{
    book::event::{CreateBook, UpdateBook},
    id::BookId,
};

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateBookRequest {
    #[garde(length(min = 1))]
    pub title: String,
    #[garde(length(min = 1))]
    pub author: String,
    #[garde(length(min = 1))]
    pub url: String,
    #[garde(length(min = 1))]
    pub title: String,
}

impl From<CreateBookRequest> for CreateBook {
    fn from(value: CreateBookRequest) -> Self {
        let CreateBookRequest { title, author, url } = value;
        Self { title, author, url }
    }
}

#[derive(new)]
pub struct UpdateBookRequestWithIds(BookId, UpdateBookRequest);

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBookRequest {
    #[garde(length(min = 1))]
    pub title: String,
    #[garde(length(min = 1))]
    pub author: String,
    #[garde(length(min = 1))]
    pub url: String,
}

impl From<UpdateBookRequest> for UpdateBook {
    fn from(value: UpdateBookRequestWithIds) -> Self {
        let UpdateBookRequestWithIds(
            book_id,
            UpdateBook {
                book_id,
                title,
                url,
                author,
            },
        ) = value;
        Self {
            book_id,
            title,
            url,
            author,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BookResponse {
    pub id: BookId,
    pub title: String,
    pub author: String,
    pub url: String,
}

impl From<Book> for BookResponse {
    fn from(value: Book) -> Self {
        let Book {
            id,
            title,
            author,
            url,
        } = value;
        Self {
            id,
            title,
            author,
            url,
        }
    }
}
