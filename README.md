                RustWall

            +------------------+
            |     Inbound      |
            | Socks5 Http Tun  |
            +--------+---------+
                     |
                     v
            +------------------+
            |   Dispatcher     |  <-- 核心调度器
            +--------+---------+
                     |
         +-----------+-----------+
         |                       |
         v                       v
   Rule Engine              Connection
                                 |
                                 v
                           OutboundManager
                          /      |       \
                         /       |        \
                    Direct   Shadowsocks  Vless

src/
│
├── app/
│   ├── mod.rs
│   ├── context.rs
│   └── builder.rs
│
├── config/
│   ├── mod.rs
│   └── config.rs
│
├── dispatcher/
│   ├── mod.rs
│   └── dispatcher.rs
│
├── dns/
│   ├── mod.rs
│   ├── cache.rs
│   └── resolver.rs
│
├── inbound/
│   └── socks5.rs
│
├── outbound/
│   ├── mod.rs
│   ├── direct.rs
│   └── manager.rs
│
├── rule/
│   ├── mod.rs
│   ├── action.rs
│   ├── rule.rs
│   ├── parser.rs
│   └── engine.rs
│
├── session/
│   ├── mod.rs
│   └── request.rs
│
├── transport/
│   ├── mod.rs
│   └── relay.rs
│
├── handler.rs
└── main.rs                   