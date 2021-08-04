# daily-warm-rs
daily-warm Rust version
> 每天定时发邮件给你关心的人, 内容包含天气, one 的一句话, 一句英语, 一首古诗

## Usage

```bash
# 1. Config `.env` file in root dir, use toml format
username = "user@qq.com"
password = "********"  # 邮箱授权码(QQ邮箱)
host = "smtp.qq.com"
port = 465 # 默认
subject = "每日一暖, 温情一生"
from = "天心<790956404@qq.com>"
to = {email = "天心<user@qq.com>", local = "shaanxi/xian"}

# 2. run
cargo run
```