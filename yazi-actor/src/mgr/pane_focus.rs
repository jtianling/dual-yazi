use anyhow::Result;
use yazi_macro::{act, render, succ};
use yazi_parser::mgr::PaneFocusOpt;
use yazi_shared::data::Data;

use crate::{Actor, Ctx};

pub struct PaneFocus;

impl Actor for PaneFocus {
	type Options = PaneFocusOpt;

	const NAME: &str = "pane_focus";

	fn act(cx: &mut Ctx, opt: Self::Options) -> Result<Data> {
		let tabs = cx.tabs_mut();
		let idx = (!opt.left) as usize;

		if idx == tabs.cursor || idx >= tabs.len() {
			succ!();
		}

		tabs.set_idx(idx);
		let cx = &mut Ctx::renew(cx);

		act!(mgr:refresh, cx)?;
		act!(mgr:peek, cx, true)?;
		act!(app:title, cx).ok();
		succ!(render!());
	}
}
