use anyhow::Result;
use yazi_actor::Ctx;
use yazi_boot::BOOT;
use yazi_macro::{act, succ};
use yazi_parser::{VoidOpt, mgr::CdSource};
use yazi_shared::{data::Data, strand::StrandLike, url::UrlLike};

use crate::Actor;

pub struct Bootstrap;

impl Actor for Bootstrap {
	type Options = VoidOpt;

	const NAME: &str = "bootstrap";

	fn act(cx: &mut Ctx, _: Self::Options) -> Result<Data> {
		let count = BOOT.files.len().max(2);
		cx.mgr.tabs.resize_with(count, Default::default);

		for (i, file) in BOOT.files.iter().enumerate().rev() {
			cx.tab = i;
			if file.is_empty() {
				act!(mgr:cd, cx, (BOOT.cwds[i].clone(), CdSource::Tab))?;
			} else if let Ok(u) = BOOT.cwds[i].try_join(file) {
				act!(mgr:reveal, cx, (u, CdSource::Tab))?;
			}
		}

		if BOOT.files.len() < 2 {
			cx.tab = 1;
			act!(mgr:cd, cx, (BOOT.cwds[0].clone(), CdSource::Tab))?;
		}

		cx.tab = 0;
		succ!();
	}
}
