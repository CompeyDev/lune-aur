use mlua::prelude::*;

pub mod datatypes;
pub mod document;
pub mod instance;

#[cfg(test)]
mod tests;

fn make_dt<F>(lua: &Lua, f: F) -> LuaResult<LuaValue>
where
    F: Fn(&Lua, &LuaTable) -> LuaResult<()>,
{
    let tab = lua.create_table()?;
    f(lua, &tab)?;
    tab.set_readonly(true);
    Ok(LuaValue::Table(tab))
}

#[rustfmt::skip]
fn make_all_datatypes(lua: &Lua) -> LuaResult<Vec<(&'static str, LuaValue)>> {
	use datatypes::types::*;
    Ok(vec![
		// Classes
        ("BrickColor",            make_dt(lua, BrickColor::make_table)?),
        ("Color3",                make_dt(lua, Color3::make_table)?),
        ("ColorSequence",         make_dt(lua, ColorSequence::make_table)?),
        ("ColorSequenceKeypoint", make_dt(lua, ColorSequenceKeypoint::make_table)?),
        ("UDim",                  make_dt(lua, UDim::make_table)?),
        ("UDim2",                 make_dt(lua, UDim2::make_table)?),
        ("Vector2",               make_dt(lua, Vector2::make_table)?),
        ("Vector2int16",          make_dt(lua, Vector2int16::make_table)?),
        ("Vector3",               make_dt(lua, Vector3::make_table)?),
        ("Vector3int16",          make_dt(lua, Vector3int16::make_table)?),
		// Singletons
        ("Enum", LuaValue::UserData(Enums::make_singleton(lua)?)),
    ])
}

pub fn module(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    for (name, tab) in make_all_datatypes(lua)? {
        exports.set(name, tab)?;
    }
    Ok(exports)
}
