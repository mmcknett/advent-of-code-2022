#[derive(Debug, Eq, PartialEq)]
pub enum Command {
  MakeDir,
  FileSize(u64),
  FinishDir,
  Ignore
}