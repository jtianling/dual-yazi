use mlua::{ExternalError, FromLua, IntoLua, Lua, Value};
use yazi_shared::event::ActionCow;

#[derive(Debug)]
pub struct RedoOpt;

impl From<ActionCow> for RedoOpt {
	fn from(_: ActionCow) -> Self { Self }
}

impl FromLua for RedoOpt {
	fn from_lua(_: Value, _: &Lua) -> mlua::Result<Self> { Err("unsupported".into_lua_err()) }
}

impl IntoLua for RedoOpt {
	fn into_lua(self, _: &Lua) -> mlua::Result<Value> { Err("unsupported".into_lua_err()) }
}
