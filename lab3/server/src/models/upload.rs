use serde::Serialize;

#[derive(Serialize)]
pub struct UploadPayload {
    pub status: UploadStatus,
    pub download_link: Option<String>,
}

impl UploadPayload {
    pub fn started() -> Self {
        UploadPayload {
            status: UploadStatus::Started,
            download_link: None,
        }
    }

    pub fn completed(download_link: String) -> Self {
        UploadPayload {
            status: UploadStatus::Completed,
            download_link: Some(download_link),
        }
    }

    pub fn error() -> Self {
        UploadPayload {
            status: UploadStatus::Error,
            download_link: None,
        }
    }
}

#[derive(Serialize)]
pub enum UploadStatus {
    Started,
    Completed,
    Error,
}
