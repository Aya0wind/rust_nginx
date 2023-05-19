use mlua::Lua;
thread_local! {
    static LUA_ENV:mlua::Lua=mlua::Lua::new();
}

struct LuaEnv{

}

impl LuaEnv{

    pub fn start<R,F:FnMut(&Lua)->R>(f:F)->R{
        LUA_ENV.with(f)
    }

}