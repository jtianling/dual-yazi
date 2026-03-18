use anyhow::Result;
use yazi_fs::{File, FilesOp};
use yazi_macro::succ;
use yazi_parser::mgr::RedoOpt;
use yazi_shared::{UndoOp, data::Data, url::UrlLike};
use yazi_vfs::{VfsFile, provider};
use yazi_watcher::WATCHER;

use crate::{Actor, Ctx};

pub struct Redo;

impl Actor for Redo {
	type Options = RedoOpt;

	const NAME: &str = "redo";

	fn act(cx: &mut Ctx, _opt: Self::Options) -> Result<Data> {
		let Some(entry) = cx.mgr.undo.redo() else { succ!() };

		tokio::spawn(async move {
			let _permit = WATCHER.acquire().await.unwrap();
			match entry.op {
				UndoOp::Rename { ref old, ref new } => {
					Self::redo_rename(old, new).await.ok();
				}
				UndoOp::Create { ref target, is_dir } => {
					Self::redo_create(target, is_dir).await.ok();
				}
				UndoOp::Copy { created: _ } => {
					// Cannot redo a copy without original sources; skip
				}
				UndoOp::Move { ref pairs } => {
					Self::redo_move(pairs).await.ok();
				}
			}
		});
		succ!();
	}
}

impl Redo {
	async fn redo_rename(
		old: &yazi_shared::url::UrlBuf,
		new: &yazi_shared::url::UrlBuf,
	) -> Result<()> {
		provider::rename(old, new).await?;

		let Some((old_p, old_n)) = old.pair() else { return Ok(()) };
		let Some((new_p, new_n)) = new.pair() else { return Ok(()) };

		let file = File::new(new).await?;
		if old_p == new_p {
			FilesOp::Upserting(new_p.into(), [(old_n.into(), file)].into()).emit();
		} else {
			FilesOp::Deleting(old_p.into(), [old_n.into()].into()).emit();
			FilesOp::Upserting(new_p.into(), [(new_n.into(), file)].into()).emit();
		}
		Ok(())
	}

	async fn redo_create(
		target: &yazi_shared::url::UrlBuf,
		is_dir: bool,
	) -> Result<()> {
		if is_dir {
			provider::create_dir_all(target).await?;
		} else {
			if let Some(parent) = target.parent() {
				provider::create_dir_all(parent).await.ok();
			}
			provider::create(target).await?;
		}

		if let Some((parent, urn)) = target.pair() {
			if let Ok(file) = File::new(target).await {
				FilesOp::Upserting(parent.into(), [(urn.into(), file)].into()).emit();
			}
		}
		Ok(())
	}

	async fn redo_move(
		pairs: &[(yazi_shared::url::UrlBuf, yazi_shared::url::UrlBuf)],
	) -> Result<()> {
		for (from, to) in pairs {
			if provider::rename(from, to).await.is_ok() {
				if let Some((from_p, from_n)) = from.pair() {
					FilesOp::Deleting(from_p.into(), [from_n.into()].into()).emit();
				}
				if let Some((to_p, to_n)) = to.pair() {
					if let Ok(file) = File::new(to).await {
						FilesOp::Upserting(to_p.into(), [(to_n.into(), file)].into()).emit();
					}
				}
			}
		}
		Ok(())
	}
}
