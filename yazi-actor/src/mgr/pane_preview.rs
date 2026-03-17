use anyhow::Result;
use yazi_macro::{act, render, succ};
use yazi_parser::mgr::PanePreviewOpt;
use yazi_shared::data::Data;

use crate::{Actor, Ctx};

pub struct PanePreview;

impl Actor for PanePreview {
	type Options = PanePreviewOpt;

	const NAME: &str = "pane_preview";

	fn act(cx: &mut Ctx, _: Self::Options) -> Result<Data> {
		cx.tabs_mut().preview_pane = !cx.tabs().preview_pane;

		act!(mgr:peek, cx, true)?;
		act!(app:title, cx).ok();
		succ!(render!());
	}
}
