use anyhow::Result;
use yazi_macro::{act, render, succ};
use yazi_parser::mgr::PaneOnlyOpt;
use yazi_shared::data::Data;

use crate::{Actor, Ctx};

pub struct PaneOnly;

impl Actor for PaneOnly {
	type Options = PaneOnlyOpt;

	const NAME: &str = "pane_only";

	fn act(cx: &mut Ctx, _: Self::Options) -> Result<Data> {
		cx.tabs_mut().single_pane = !cx.tabs().single_pane;

		act!(mgr:peek, cx, true)?;
		act!(app:title, cx).ok();
		succ!(render!());
	}
}
