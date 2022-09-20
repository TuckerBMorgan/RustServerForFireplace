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
        implement_lua_push!($ty, $cb);
        implement_lua_read!($ty);
    }
}

#[macro_export]
macro_rules! implement_enum_and_unfold {
    ($($enum_id:ident,)+) => (

        #[derive(Clone, Debug)]
        pub enum ERuneType {
            $(
                $enum_id($enum_id),
            )*
        }

        impl ERuneType {
            pub fn unfold(&self) -> Box<dyn Rune> {
                match *self {
                    $(
                        ERuneType::$enum_id(ref val) => {
                            return val.into_box();
                        },
                    )*
                }
            }
        }
        implement_for_lua!(ERuneType, |mut metatable| {});
    )
}
