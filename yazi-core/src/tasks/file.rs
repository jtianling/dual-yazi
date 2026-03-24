use std::time::Duration;

use hashbrown::HashSet;
use tracing::debug;
use yazi_parser::notify::{PushLevel, PushOpt};
use yazi_proxy::NotifyProxy;
use yazi_shared::{CompletionToken, url::{UrlBuf, UrlBufCov, UrlLike}};

use super::Tasks;
use crate::mgr::Yanked;

impl Tasks {
	pub fn file_cut(&self, src: &Yanked, dest: &UrlBuf, force: bool) {
		let mut tokens = Vec::new();
		for u in src.iter() {
			let Some(Ok(to)) = u.name().map(|n| dest.try_join(n)) else {
				debug!("file_cut: cannot join {u:?} with {dest:?}");
				continue;
			};
			if force && *u == to {
				debug!("file_cut: same file, skip {to:?}");
			} else {
				tokens.push(self.scheduler.file_cut(u.0.clone(), to, force));
			}
		}
		Self::spawn_notify(tokens, "Move", force);
	}

	pub fn file_copy(&self, src: &Yanked, dest: &UrlBuf, force: bool, follow: bool) {
		let mut tokens = Vec::new();
		for u in src.iter() {
			let Some(Ok(to)) = u.name().map(|n| dest.try_join(n)) else {
				debug!("file_copy: cannot join {u:?} with {dest:?}");
				continue;
			};
			if force && *u == to {
				debug!("file_copy: same file, skip {to:?}");
			} else {
				tokens.push(self.scheduler.file_copy(u.0.clone(), to, force, follow));
			}
		}
		Self::spawn_notify(tokens, "Copy", force);
	}

	pub fn file_link(&self, src: &HashSet<UrlBufCov>, dest: &UrlBuf, relative: bool, force: bool) {
		for u in src {
			let Some(Ok(to)) = u.name().map(|n| dest.try_join(n)) else {
				debug!("file_link: cannot join {u:?} with {dest:?}");
				continue;
			};
			if force && *u == to {
				debug!("file_link: same file, skip {to:?}");
			} else {
				self.scheduler.file_link(u.0.clone(), to, relative, force);
			}
		}
	}

	pub fn file_hardlink(&self, src: &HashSet<UrlBufCov>, dest: &UrlBuf, force: bool, follow: bool) {
		for u in src {
			let Some(Ok(to)) = u.name().map(|n| dest.try_join(n)) else {
				debug!("file_hardlink: cannot join {u:?} with {dest:?}");
				continue;
			};
			if force && *u == to {
				debug!("file_hardlink: same file, skip {to:?}");
			} else {
				self.scheduler.file_hardlink(u.0.clone(), to, force, follow);
			}
		}
	}

	pub fn file_remove(&self, targets: Vec<UrlBuf>, permanently: bool) {
		for u in targets {
			if permanently {
				self.scheduler.file_delete(u);
			} else {
				self.scheduler.file_trash(u);
			}
		}
	}

	fn spawn_notify(tokens: Vec<CompletionToken>, op: &'static str, force: bool) {
		if tokens.is_empty() {
			return;
		}

		let total = tokens.len();
		tokio::spawn(async move {
			let mut success = 0usize;
			for token in tokens {
				if token.future().await {
					success += 1;
				}
			}

			let failed = total - success;
			let prefix = if force { "Force " } else { "" };

			if failed == 0 {
				NotifyProxy::push(PushOpt {
					title:   op.into(),
					content: format!(
						"{prefix}{} {total} item(s) successfully",
						if op == "Copy" { "copied" } else { "moved" }
					),
					level:   PushLevel::Info,
					timeout: Duration::from_secs(3),
				});
			} else {
				NotifyProxy::push_warn(
					op,
					format!(
						"{prefix}{} {success} of {total} item(s), {failed} failed",
						if op == "Copy" { "copied" } else { "moved" }
					),
				);
			}
		});
	}
}
