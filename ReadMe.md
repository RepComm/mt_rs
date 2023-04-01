# mt_rs


Cancelled until vscode and rust can play nicely together
Intellisense is broken, and I'm not going to be learning fast enough without that. See you next year folks.


Minetest clone project, written in rust

This project tries to behave like minetest does, at least as far as the modding LUA API is concerned.

Just a note: I have no experience in rust, so this should be fun.

## State
Basic triangle test code working, as well as LUA code execution
![img](./example.png)

## Dependencies

- [bevy](https://github.com/bevyengine/bevy)
- [luajit-rs](https://gitlab.com/Dreae/luajit-rs)

- [bevy system dependencies](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md)

- [luajit system installation](https://luajit.org/install.html)
  - brief rundown:
    `git clone https://github.com/LuaJIT/LuaJIT`
    `make`
    `sudo make install`