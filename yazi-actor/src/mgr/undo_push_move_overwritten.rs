use anyhow::Result;
use yazi_macro::succ;
use yazi_parser::mgr::UndoPushMoveOverwrittenOpt;
use yazi_shared::data::Data;

use crate::{Actor, Ctx};

pub struct UndoPushMoveOverwritten;

impl Actor for UndoPushMoveOverwritten {
	type Options = UndoPushMoveOverwrittenOpt;

	const NAME: &str = "undo_push_move_overwritten";

	fn act(cx: &mut Ctx, opt: Self::Options) -> Result<Data> {
		if let (Some(original), Some(trash_path)) = (opt.original, opt.trash_path) {
			cx.mgr.undo.push_move_overwritten(original, trash_path);
		}
		succ!();
	}
}
