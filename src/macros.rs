#[allow(unused_imports)]
use hlua;

macro_rules! implement_lua_userdata_read {
    ($ty:ty) => {
        impl<'lua, L> hlua::LuaRead<L> for $ty
            where L: hlua::AsMutLua<'lua>
            {
                fn lua_read_at_position(lua: L, index: i32) -> Result<$ty, L> {
                let val: Result<hlua::UserdataOnStack<$ty, _>, _> =
                    hlua::LuaRead::lua_read_at_position(lua, index);
                val.map(|d| d.clone())
            }
        }
    }
}

#[macro_export]
macro_rules! implement_for_lua {
    ($ty:ty, $cb:expr) => {
        implement_lua_userdata_read!($ty);
        implement_lua_read!($ty);
        implement_lua_push!($ty, $cb);
    }
}
