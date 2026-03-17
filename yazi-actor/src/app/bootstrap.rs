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
		cx.pane = 0;
		cx.tab = 0;
		if let Some(file) = BOOT.files.first() {
			if file.is_empty() {
				act!(mgr:cd, cx, (BOOT.cwds[0].clone(), CdSource::Tab))?;
			} else if let Ok(u) = BOOT.cwds[0].try_join(file) {
				act!(mgr:reveal, cx, (u, CdSource::Tab))?;
			}
		} else {
			act!(mgr:cd, cx, (BOOT.cwds[0].clone(), CdSource::Tab))?;
		}

		cx.pane = 1;
		cx.tab = 0;
		if BOOT.files.len() >= 2 {
			let file = &BOOT.files[1];
			if file.is_empty() {
				act!(mgr:cd, cx, (BOOT.cwds[1].clone(), CdSource::Tab))?;
			} else if let Ok(u) = BOOT.cwds[1].try_join(file) {
				act!(mgr:reveal, cx, (u, CdSource::Tab))?;
			}
		} else {
			act!(mgr:cd, cx, (BOOT.cwds[0].clone(), CdSource::Tab))?;
		}

		cx.pane = 0;
		cx.tab = 0;
		succ!();
	}
}
