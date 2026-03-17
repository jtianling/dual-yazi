use anyhow::Result;
use yazi_macro::{act, render, succ};
use yazi_parser::mgr::TabSwitchOpt;
use yazi_shared::data::Data;

use crate::{Actor, Ctx};

pub struct TabSwitch;

impl Actor for TabSwitch {
	type Options = TabSwitchOpt;

	const NAME: &str = "tab_switch";

	fn act(cx: &mut Ctx, opt: Self::Options) -> Result<Data> {
		let pane = cx.tabs_mut().active_pane_mut();
		let idx = if opt.relative {
			opt.step.saturating_add_unsigned(pane.cursor).rem_euclid(pane.len() as _) as _
		} else {
			opt.step as usize
		};

		if idx == pane.cursor || idx >= pane.len() {
			succ!();
		}

		pane.set_idx(idx);
		let cx = &mut Ctx::renew(cx);

		act!(mgr:refresh, cx)?;
		act!(mgr:peek, cx, true)?;
		act!(app:title, cx).ok();
		succ!(render!());
	}
}
