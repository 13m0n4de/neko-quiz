<div align="center">

# NekoQuiz

NekoQuiz 是一个 CTF 问答题通用框架，对 [USTC Hackergame 猫咪问答](https://github.com/USTC-Hackergame/hackergame2023-writeups/blob/master/official/%E7%8C%AB%E5%92%AA%E5%B0%8F%E6%B5%8B/README.md) 的仿制。

Rust 编写，前端使用 [Yew](https://yew.rs/) + [Bootstrap](https://getbootstrap.com/) ，后端使用 [Axum](https://github.com/tokio-rs/axum) ，一键部署至各比赛平台。

![GitHub License](https://img.shields.io/badge/license-MIT-green)
![GitHub Repo stars](https://img.shields.io/github/stars/13m0n4de/neko-quiz)

[预览](#预览) •
[安装](#安装) •
[配置](#配置) •
[部署](#部署) •
[帮助](#帮助)

</div>

## 预览

![demo-1](assets/demo-1.png)
![demo-2](assets/demo-2.png)

## 安装

### 预构建二进制文件

你可以在 [Github release](https://github.com/13m0n4de/neko-quiz/release) 页面找到自动构建的二进制文件，下载对应系统架构的文件解压即可。

### Docker 镜像

```
$ docker pull https://ghcr.io/13m0n4de/neko-quiz/todo:latest
```

### 从源文件构建

如果选择从源文件构建，先要确保拥有 Rust 工具链，并安装 [Trunk](https://github.com/thedodd/trunk)：

```
$ cargo install trunk  # 从源码安装
$ cargo binstall trunk  # 或使用 cargo-binstall 安装二进制文件
```

克隆源代码：

```
$ git clone https://github.com/13m0n4de/neko-quiz/
```

使用 [build.sh](build.sh) 可以快速编译：

```
$ ./build.sh
```

前端文件输出在 `dist/` 后端文件输出在 `target/release/backend`

## 配置

项目根目录有一份配置示例：[config.json](config.json)

```json
{
  "title": "猫咪小测",
  "questions": [
    {
      "text": "想要借阅世界图书出版公司出版的《A Classical Introduction To Modern Number Theory 2nd ed.》，应当前往中国科学技术大学西区图书馆的哪一层？",
      "points": 30,
      "hint": "是一个非负整数。",
      "answer": ["12"]
    },
    {
      "text": "今年 arXiv 网站的天体物理版块上有人发表了一篇关于「可观测宇宙中的鸡的密度上限」的论文，请问论文中作者计算出的鸡密度函数的上限为 10 的多少次方每立方秒差距？",
      "points": 30,
      "hint": "是一个非负整数。",
      "answer": ["23"]
    },
    {
      "text": "为了支持 TCP BBR 拥塞控制算法，在<b>编译</b> Linux 内核时应该配置好哪一条内核选项？",
      "points": 20,
      "hint": "输入格式为 CONFIG_XXXXX，如 CONFIG_SCHED_SMT。",
      "answer": ["CONFIG_TCP_CONG_BBR"]
    },
    {
      "text": "🥒🥒🥒：「我……从没觉得写类型标注有意思过」。在一篇论文中，作者给出了能够让 Python 的类型检查器 MyPY mypy 陷入死循环的代码，并证明 Python 的类型检查和停机问题一样困难。请问这篇论文发表在今年的哪个学术会议上？",
      "points": 20,
      "hint": "会议的大写英文简称，比如 ISCA、CCS、ICML。",
      "answer": ["ECOOP"]
    }
  ],
  "flag": {
    "flag_env": "FLAG",
    "flag_file": "/flag",
    "flag_static": "flag{neko_quiz_static_flag}"
  },
  "message": {
    "incorrect": "没有全部答对，不能给你 FLAG 哦。",
    "correct": "🎉🎉🎉 $FLAG 🎉🎉🎉"
  }
}
```

- `title`：标题
- `questions`：题目列表，按顺序填写，不限制数量
  - `text`：正文，可以使用 HTML 标签
  - `points`：分数（ Flag 不按照分数计算，分数只是显示效果）
  - `hint`：提示，可以使用 HTML 标签
  - `answer`：答案，可以配置多个，与任意一个相等即认为回答正确
- `flag`：Flag 获取方式，默认顺序：环境变量 -> 文件 -> 静态字符串
  - `flag_env`：环境变量
  - `flag_file`：文件路径
  - `flag_static`：静态字符串
- `message`：返回消息，可以使用 HTML 标签
  - `incorrect`：回答错误时的消息
  - `correct`：回答正确时的消息，使用 `$FLAG` 占位符表示 Flag 值

## 部署

默认端口为 `3000`

### Docker 镜像

确保挂载的配置文件 `config.json` 路径正确。

使用环境变量作为 Flag：

```
$ docker run -d --rm -p 3000:3000 -v ./config.json:/config.json -e FLAG='flag{13m0n4de}' --name neko-quiz neko-quiz:latest
```

使用文件内容作为 Flag：

```
$ docker run -d --rm -p 3000:3000 -v ./config.json:/config.json -v ./flag:/flag --name neko-quiz neko-quiz:latest
```

或者使用 `docker-compose`

```
$ docker-compose up
```

### 本地部署

编译 Release 版本并启动：

```
$ ./prod.sh
```

构建本地 Docker 镜像：

```
$ docker build . -t neko-quiz
```

## 帮助

服务端命令行参数：

```
$ target/release/backend --help
backend for neko quiz

Usage: backend [OPTIONS]

Options:
  -l, --log <LOG_LEVEL>          [default: debug]
  -a, --addr <ADDR>              [default: localhost]
  -p, --port <PORT>              [default: 3000]
  -c, --config <CONFIG>          [default: config.json]
      --static-dir <STATIC_DIR>  [default: ./dist]
  -h, --help                     Print help
```

## 使用案例

- [SVUCTF/SVUCTF-WINTER-2023 猫娘问答](https://github.com/SVUCTF/SVUCTF-WINTER-2023/tree/main/challenges/misc/neko_quiz)

## 许可证

该项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解更多细节。

## 相关项目

- [USTC-Hackergame 猫咪小测](https://github.com/USTC-Hackergame/hackergame2023-writeups/blob/master/official/%E7%8C%AB%E5%92%AA%E5%B0%8F%E6%B5%8B/README.md)
- [rksm/axum-yew-setup](https://github.com/rksm/axum-yew-setup/)