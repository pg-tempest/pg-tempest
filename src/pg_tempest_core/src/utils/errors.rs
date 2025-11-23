use derive_more::Debug as DebugV2;
use std::error::Error;
use std::sync::Arc;
use thiserror::Error;

pub type BoxDynError = Box<dyn Error + Send + Sync>;
pub type ArcDynError = Arc<dyn Error + Send + Sync>;

#[derive(Error, DebugV2)]
#[error(transparent)]
#[debug("{_0:?}")]
pub struct ErrorArcDynError(#[from] pub Arc<dyn Error + Send + Sync>);

pub trait ErrorExt {
    type Ok;

    fn box_err(self) -> Result<Self::Ok, BoxDynError>;
}

impl<T, E: Into<BoxDynError>> ErrorExt for Result<T, E> {
    type Ok = T;

    fn box_err(self) -> Result<T, BoxDynError> {
        self.map_err(|e| e.into())
    }
}
