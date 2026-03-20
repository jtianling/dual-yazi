use mlua::{ExternalError, FromLua, IntoLua, Lua, Value};
use yazi_shared::{event::ActionCow, url::UrlBuf};

#[derive(Debug)]
pub struct UndoPushCopyOverwrittenOpt {
	pub original:   Option<UrlBuf>,
	pub trash_path: Option<UrlBuf>,
}

impl From<ActionCow> for UndoPushCopyOverwrittenOpt {
	fn from(mut a: ActionCow) -> Self {
		Self { original: a.take_any("original"), trash_path: a.take_any("trash_path") }
	}
}

impl FromLua for UndoPushCopyOverwrittenOpt {
	fn from_lua(_: Value, _: &Lua) -> mlua::Result<Self> { Err("unsupported".into_lua_err()) }
}

impl IntoLua for UndoPushCopyOverwrittenOpt {
	fn into_lua(self, _: &Lua) -> mlua::Result<Value> { Err("unsupported".into_lua_err()) }
}
