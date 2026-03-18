use anyhow::Result;
use yazi_macro::succ;
use yazi_parser::mgr::UndoPushOpt;
use yazi_shared::data::Data;

use crate::{Actor, Ctx};

pub struct UndoPush;

impl Actor for UndoPush {
	type Options = UndoPushOpt;

	const NAME: &str = "undo_push";

	fn act(cx: &mut Ctx, opt: Self::Options) -> Result<Data> {
		if let Some(op) = opt.op {
			cx.mgr.undo.push(op);
		}
		succ!();
	}
}
