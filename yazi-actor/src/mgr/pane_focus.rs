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
		let idx = (!opt.left) as usize;

		if idx == cx.tabs().active_pane {
			succ!();
		}

		cx.tabs_mut().set_active_pane(idx);
		let cx = &mut Ctx::renew(cx);

		act!(mgr:refresh, cx)?;
		act!(mgr:peek, cx, true)?;
		act!(app:title, cx).ok();
		succ!(render!());
	}
}
