use anyhow::Result;
use yazi_macro::succ;
use yazi_parser::mgr::UndoPushCopyPairOpt;
use yazi_shared::data::Data;

use crate::{Actor, Ctx};

pub struct UndoPushCopyPair;

impl Actor for UndoPushCopyPair {
	type Options = UndoPushCopyPairOpt;

	const NAME: &str = "undo_push_copy_pair";

	fn act(cx: &mut Ctx, opt: Self::Options) -> Result<Data> {
		if let (Some(from), Some(to)) = (opt.from, opt.to) {
			cx.mgr.undo.push_copy_pair(from, to);
		}
		succ!();
	}
}
