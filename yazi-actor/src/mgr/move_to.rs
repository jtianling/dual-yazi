use anyhow::Result;
use yazi_core::mgr::Yanked;
use yazi_macro::{act, succ};
use yazi_parser::mgr::MoveToOpt;
use yazi_shared::{UndoOp, data::Data, url::{UrlBufCov, UrlLike}};

use crate::{Actor, Ctx};

pub struct MoveTo;

impl Actor for MoveTo {
	type Options = MoveToOpt;

	const NAME: &str = "move_to";

	fn act(cx: &mut Ctx, opt: Self::Options) -> Result<Data> {
		act!(mgr:escape_visual, cx)?;

		let yanked =
			Yanked::new(true, cx.tab().selected_or_hovered().cloned().map(UrlBufCov).collect());
		if yanked.is_empty() {
			succ!();
		}

		let dest = cx.tabs().other().cwd().clone();

		let pairs: Vec<_> = yanked
			.iter()
			.filter_map(|u| {
				u.name().map(|n| dest.try_join(n)).and_then(|r| r.ok()).map(|to| ((**u).clone(), to))
			})
			.collect();

		cx.core.tasks.file_cut(&yanked, &dest, opt.force);
		cx.mgr.undo.push(UndoOp::Move { pairs });
		act!(mgr:escape_select, cx)?;
		act!(mgr:unyank, cx)
	}
}
