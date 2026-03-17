use anyhow::Result;
use yazi_dds::Pubsub;
use yazi_macro::{err, render, succ};
use yazi_parser::ArrowOpt;
use yazi_shared::data::Data;

use crate::{Actor, Ctx};

pub struct TabSwap;

impl Actor for TabSwap {
	type Options = ArrowOpt;

	const NAME: &str = "tab_swap";

	fn act(cx: &mut Ctx, opt: Self::Options) -> Result<Data> {
		let pane = cx.tabs_mut().active_pane_mut();

		let new = opt.step.add(pane.cursor, pane.len(), 0);
		if new == pane.cursor {
			succ!();
		}

		pane.items.swap(pane.cursor, new);
		pane.cursor = new;

		err!(Pubsub::pub_after_tab(cx.tabs().active().id));
		succ!(render!());
	}
}
