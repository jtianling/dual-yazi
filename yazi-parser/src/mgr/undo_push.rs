use mlua::{ExternalError, FromLua, IntoLua, Lua, Value};
use yazi_shared::{UndoOp, event::ActionCow};

#[derive(Debug)]
pub struct UndoPushOpt {
	pub op: Option<UndoOp>,
}

impl From<ActionCow> for UndoPushOpt {
	fn from(mut a: ActionCow) -> Self { Self { op: a.take_any("op") } }
}

impl FromLua for UndoPushOpt {
	fn from_lua(_: Value, _: &Lua) -> mlua::Result<Self> { Err("unsupported".into_lua_err()) }
}

impl IntoLua for UndoPushOpt {
	fn into_lua(self, _: &Lua) -> mlua::Result<Value> { Err("unsupported".into_lua_err()) }
}
