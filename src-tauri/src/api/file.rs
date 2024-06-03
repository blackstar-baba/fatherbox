use std::io;

use axum::{BoxError, Json};
use axum::body::{Body, Bytes};
use axum::extract::{Multipart, Path};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{Html, IntoResponse};
use futures::{Stream, TryStreamExt};
use tokio::fs::File;
use tokio::io::BufWriter;
use tokio_util::io::{ReaderStream, StreamReader};

use crate::{AppResponse, RESPONSE_CODE_SUCCESS};

// Handler that returns HTML for a multipart form.
pub async fn show_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head>
                <title>Upload something!</title>
            </head>
            <body>
                <form action="/" method="post" enctype="multipart/form-data">
                    <div>
                        <label>
                            Upload file:
                            <input type="file" name="file" multiple>
                        </label>
                    </div>

                    <div>
                        <input type="submit" value="Upload files">
                    </div>
                </form>
            </body>
        </html>
        "#,
    )
}

// Handler that accepts a multipart form upload and streams each field to a file.
pub async fn upload_file(mut multipart: Multipart) -> Json<AppResponse<Vec<String>>> {
    let mut file_name = "".to_string();
    while let Ok(Some(field)) = multipart.next_field().await {
        file_name = if let Some(file_name) = field.file_name() {
            file_name.to_owned()
        } else {
            continue;
        };

        stream_to_file(&file_name, field).await.unwrap();
    }
    Json(AppResponse {
        code: RESPONSE_CODE_SUCCESS,
        r#type: "".to_string(),
        message: "".to_string(),
        result: vec![file_name.to_string()],
    })
}

// Save a `Stream` to a file
async fn stream_to_file<S, E>(path: &str, stream: S) -> Result<(), (StatusCode, String)>
    where
        S: Stream<Item = Result<Bytes, E>>,
        E: Into<BoxError>,
{
    if !path_is_valid(path) {
        return Err((StatusCode::BAD_REQUEST, "Invalid path".to_owned()));
    }

    async {
        // Convert the stream into an `AsyncRead`.
        let body_with_io_error = stream.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
        let body_reader = StreamReader::new(body_with_io_error);
        futures::pin_mut!(body_reader);

        // Create the file. `File` implements `AsyncWrite`.
        let workspace = "/Users/blackstar/.fatherbox/workspace";
        if !tokio::fs::try_exists(workspace).await.unwrap() {
            tokio::fs::create_dir(workspace)
                .await
                .expect("failed to create `workspace` directory");
        }
        let path = std::path::Path::new(workspace).join(path);
        let mut file = BufWriter::new(File::create(path).await?);

        // Copy the body into the file.
        tokio::io::copy(&mut body_reader, &mut file).await?;

        Ok::<_, io::Error>(())
    }
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}

// to prevent directory traversal attacks we ensure the path consists of exactly one normal
// component
fn path_is_valid(path: &str) -> bool {
    let path = std::path::Path::new(path);
    let mut components = path.components().peekable();

    if let Some(first) = components.peek() {
        if !matches!(first, std::path::Component::Normal(_)) {
            return false;
        }
    }

    components.count() == 1
}

pub async fn download_file(Path(fileName): Path<String>) -> impl IntoResponse  {

    let file = File::open("/Users/blackstar/.fatherbox/workspace/logo2_no_name.png").await.unwrap();

    // convert the `AsyncRead` into a `Stream`
    let stream = ReaderStream::new(file);
    // convert the `Stream` into an `axum::body::HttpBody`

    let mut headers = HeaderMap::new();
    headers.append(header::CONTENT_TYPE,"application/octet-stream".parse().unwrap());
    headers.append(header::CONTENT_DISPOSITION, (&format!("attachment; filename={:?}", "logo2_no_name.png")).parse().unwrap());

    (headers, Body::from_stream(stream))

}