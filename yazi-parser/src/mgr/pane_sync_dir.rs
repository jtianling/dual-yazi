use mlua::{ExternalError, FromLua, IntoLua, Lua, Value};
use yazi_shared::event::ActionCow;

#[derive(Debug)]
pub struct PaneSyncDirOpt;

impl From<ActionCow> for PaneSyncDirOpt {
	fn from(_: ActionCow) -> Self { Self }
}

impl FromLua for PaneSyncDirOpt {
	fn from_lua(_: Value, _: &Lua) -> mlua::Result<Self> { Err("unsupported".into_lua_err()) }
}

impl IntoLua for PaneSyncDirOpt {
	fn into_lua(self, _: &Lua) -> mlua::Result<Value> { Err("unsupported".into_lua_err()) }
}
