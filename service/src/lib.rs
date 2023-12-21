mod base;
pub use base::errors::ServiceError;
pub use base::service::ServiceData;
pub use base::undo::UndoList;
pub use base::undo::UndoRedoStack;
pub mod client;
pub mod diary;
mod reps;
