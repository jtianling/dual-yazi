use anyhow::Result;
use yazi_macro::succ;
use yazi_parser::mgr::UndoPushCopyOverwrittenOpt;
use yazi_shared::data::Data;

use crate::{Actor, Ctx};

pub struct UndoPushCopyOverwritten;

impl Actor for UndoPushCopyOverwritten {
	type Options = UndoPushCopyOverwrittenOpt;

	const NAME: &str = "undo_push_copy_overwritten";

	fn act(cx: &mut Ctx, opt: Self::Options) -> Result<Data> {
		if let (Some(original), Some(trash_path)) = (opt.original, opt.trash_path) {
			cx.mgr.undo.push_copy_overwritten(original, trash_path);
		}
		succ!();
	}
}
