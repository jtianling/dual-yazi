use anyhow::Result;
use yazi_macro::{act, render, succ};
use yazi_parser::mgr::{CdSource, PaneSyncDirOpt};
use yazi_shared::data::Data;

use crate::{Actor, Ctx};

pub struct PaneSyncDir;

impl Actor for PaneSyncDir {
	type Options = PaneSyncDirOpt;

	const NAME: &str = "pane_sync_dir";

	fn act(cx: &mut Ctx, _: Self::Options) -> Result<Data> {
		let other = match cx.tabs().active_pane {
			0 => 1,
			1 => 0,
			_ => succ!(),
		};

		let cwd = cx.cwd().clone();

		// Switch to other pane
		cx.tabs_mut().set_active_pane(other);
		let cx = &mut Ctx::renew(cx);

		// Navigate other pane to the saved cwd
		act!(mgr:cd, cx, (cwd, CdSource::Cd))?;

		// Switch back to original pane
		let original = match other {
			0 => 1,
			_ => 0,
		};
		cx.tabs_mut().set_active_pane(original);
		let cx = &mut Ctx::renew(cx);

		act!(mgr:refresh, cx)?;
		act!(mgr:peek, cx, true)?;
		act!(app:title, cx).ok();
		succ!(render!());
	}
}
