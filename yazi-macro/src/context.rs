#[macro_export]
macro_rules! tab {
	($cx:ident) => {{
		let pane = $cx.pane;
		let tab = $cx.tab;
		&mut $cx.core.mgr.tabs.panes[pane].items[tab]
	}};
}
