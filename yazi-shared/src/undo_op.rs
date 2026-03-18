use crate::url::UrlBuf;

#[derive(Clone, Debug)]
pub enum UndoOp {
	Rename { old: UrlBuf, new: UrlBuf },
	Create { target: UrlBuf, is_dir: bool },
	Copy { created: Vec<UrlBuf> },
	Move { pairs: Vec<(UrlBuf, UrlBuf)> },
}
