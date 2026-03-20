use anyhow::Result;
use yazi_macro::succ;
use yazi_parser::mgr::UndoPushMovePairOpt;
use yazi_shared::data::Data;

use crate::{Actor, Ctx};

pub struct UndoPushMovePair;

impl Actor for UndoPushMovePair {
	type Options = UndoPushMovePairOpt;

	const NAME: &str = "undo_push_move_pair";

	fn act(cx: &mut Ctx, opt: Self::Options) -> Result<Data> {
		if let (Some(from), Some(to)) = (opt.from, opt.to) {
			cx.mgr.undo.push_move_pair(from, to);
		}
		succ!();
	}
}
