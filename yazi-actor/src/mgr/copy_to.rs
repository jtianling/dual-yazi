use anyhow::Result;
use yazi_core::mgr::Yanked;
use yazi_macro::{act, succ};
use yazi_parser::mgr::CopyToOpt;
use yazi_shared::{UndoOp, data::Data, url::{UrlBufCov, UrlLike}};

use crate::{Actor, Ctx};

pub struct CopyTo;

impl Actor for CopyTo {
	type Options = CopyToOpt;

	const NAME: &str = "copy_to";

	fn act(cx: &mut Ctx, opt: Self::Options) -> Result<Data> {
		act!(mgr:escape_visual, cx)?;

		let yanked =
			Yanked::new(false, cx.tab().selected_or_hovered().cloned().map(UrlBufCov).collect());
		if yanked.is_empty() {
			succ!();
		}

		let dest = cx.tabs().other().cwd().clone();

		let pairs: Vec<_> = yanked
			.iter()
			.filter_map(|u| u.name().map(|n| dest.try_join(n)).and_then(|r| r.ok()).map(|to| ((**u).clone(), to)))
			.collect();

		cx.core.tasks.file_copy(&yanked, &dest, opt.force, false);
		cx.mgr.undo.push(UndoOp::Copy { pairs });
		act!(mgr:escape_select, cx)
	}
}
