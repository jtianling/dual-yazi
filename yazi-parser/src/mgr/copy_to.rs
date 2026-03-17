use mlua::{ExternalError, FromLua, IntoLua, Lua, Value};
use yazi_shared::event::ActionCow;

#[derive(Debug)]
pub struct CopyToOpt {
	pub force: bool,
}

impl From<ActionCow> for CopyToOpt {
	fn from(a: ActionCow) -> Self { Self { force: a.bool("force") } }
}

impl FromLua for CopyToOpt {
	fn from_lua(_: Value, _: &Lua) -> mlua::Result<Self> { Err("unsupported".into_lua_err()) }
}

impl IntoLua for CopyToOpt {
	fn into_lua(self, _: &Lua) -> mlua::Result<Value> { Err("unsupported".into_lua_err()) }
}
