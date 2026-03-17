use anyhow::Result;
use yazi_macro::{act, render, succ};
use yazi_parser::mgr::PaneSwitchOpt;
use yazi_shared::data::Data;

use crate::{Actor, Ctx};

pub struct PaneSwitch;

impl Actor for PaneSwitch {
	type Options = PaneSwitchOpt;

	const NAME: &str = "pane_switch";

	fn act(cx: &mut Ctx, _: Self::Options) -> Result<Data> {
		let idx = match cx.tabs().active_pane {
			0 => 1,
			1 => 0,
			_ => succ!(),
		};

		cx.tabs_mut().set_active_pane(idx);
		let cx = &mut Ctx::renew(cx);

		act!(mgr:refresh, cx)?;
		act!(mgr:peek, cx, true)?;
		act!(app:title, cx).ok();
		succ!(render!());
	}
}
