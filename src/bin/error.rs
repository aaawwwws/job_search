use thiserror::Error;

#[derive(Debug, Error)]
pub enum Errr {
    #[error("オープンエラー")]
    OpenError,
    #[error("ページエラー")]
    PageError,
    #[error("存在しない")]
    Null,
    #[error("ベクターエラー")]
    VecError,
}
