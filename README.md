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