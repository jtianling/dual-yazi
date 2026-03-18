use anyhow::Result;
use yazi_macro::succ;
use yazi_parser::mgr::UndoPushTrashPairOpt;
use yazi_shared::data::Data;

use crate::{Actor, Ctx};

pub struct UndoPushTrashPair;

impl Actor for UndoPushTrashPair {
	type Options = UndoPushTrashPairOpt;

	const NAME: &str = "undo_push_trash_pair";

	fn act(cx: &mut Ctx, opt: Self::Options) -> Result<Data> {
		if let (Some(original), Some(trash_path)) = (opt.original, opt.trash_path) {
			cx.mgr.undo.push_trash_pair(original, trash_path);
		}
		succ!();
	}
}
