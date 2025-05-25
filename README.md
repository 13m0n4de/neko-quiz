<div align="center">

# NekoQuiz

NekoQuiz 是一个 CTF 问答题通用框架，对 [USTC Hackergame 猫咪问答](https://github.com/USTC-Hackergame/hackergame2023-writeups/blob/master/official/%E7%8C%AB%E5%92%AA%E5%B0%8F%E6%B5%8B/README.md) 的仿制。

Rust 编写，使用 [Leptos](https://leptos.dev/) + [Axum](https://github.com/tokio-rs/axum) + [Tailwind CSS](https://tailwindcss.com/)。

![GitHub License](https://img.shields.io/badge/license-MIT-green?style=for-the-badge)
![GitHub Repo stars](https://img.shields.io/github/stars/13m0n4de/neko-quiz?style=for-the-badge)

[预览](#%E9%A2%84%E8%A7%88) •
[特性](#%E7%89%B9%E6%80%A7) •
[安装](#%E5%AE%89%E8%A3%85) •
[配置](#%E9%85%8D%E7%BD%AE) •
[部署](#%E9%83%A8%E7%BD%B2) •
[帮助](#%E5%B8%AE%E5%8A%A9)

</div>

## 预览

在线预览：[neko-quiz.onrender.com](https://neko-quiz.onrender.com)

![demo](assets/demo.png)

> [!WARNING]
>
> 本项目仍处于开发阶段，可能存在未知的安全隐患和功能缺陷，仅作为实验和学习用途，不建议直接应用于生产环境。

## 特性

- 🚀 **服务端渲染（SSR）**：快速的首屏加载
- 🛠 **灵活配置**：使用 TOML 文件轻松配置问题、答案和提示
- 🔄 **配置热重载**：支持配置文件热重载，无需重启即可更新问题
- 🔒 **多种 Flag 获取方式**：支持环境变量、文件读取和静态字符串
- 🐳 **Docker 支持**：提供 Docker 镜像和 docker-compose 配置
- 🌐 **跨平台**：提供预构建的多平台二进制文件
- 💬 **自定义文本**：支持 HTML 格式的题目文本

## 安装

### 预构建二进制文件

你可以在 [Releases](https://github.com/13m0n4de/neko-quiz/releases) 页面找到自动构建的二进制文件。

### Docker 镜像

```
docker pull ghcr.io/13m0n4de/neko-quiz
```

详细的 Docker 镜像使用说明请参考[部署](#%E9%83%A8%E7%BD%B2)章节。

### 从源代码构建

1. 确保已安装 Rust 工具链
2. 安装 [cargo-leptos](https://github.com/leptos-rs/cargo-leptos)：
    ```
    cargo install cargo-leptos
    ```
3. 克隆仓库：
    ```
    git clone https://github.com/13m0n4de/neko-quiz/
    ```
4. 构建项目
    ```
    cargo leptos build --release
    ```

构建完成后，WASM 文件将输出在 `target/site/` 目录，可执行文件位于 `target/bin-release/neko-quiz`。

> [!NOTE]
>
> 注意：从源代码构建需要安装 wasm32-unknown-unknown 目标：`rustup target add wasm32-unknown-unknown`

## 配置

项目根目录有一份配置示例：[config.toml](config.toml)

```toml
[general]
title = "猫咪问答"
return_score = true

[[questions]]
text = "想要在苏州图书馆借阅 <i>Engineering a Compiler</i>，需要到哪个分馆的哪个馆藏地点？"
points = 20
hint = "格式：所在分馆-所在馆藏地点，例如 中心馆-西文书库。（如有多个，任意一个即可）"
answers = ["苏图-北馆书库", "苏图-设计图书馆"]

[[questions]]
text = """「你们这些搞计算机的，就是喜欢卖弄学问，数个数还得从零开始」。
其实编号从零开始是有道理的，一位计算机科学家还专门写过一篇简短的<b>手写稿</b>来解释，
请问这个手写稿的编写日期是？
"""
points = 20
hint = "格式：YYYY-MM-DD ，如 2024-02-05"
answers = ["1982-08-11"]

[[questions]]
text = "CVE-2023-45853 的修复补丁中，文件名长度被限制为多少位（bit）？"
points = 20
hint = "格式：整数，如 32"
answers = ["16"]

[[questions]]
text = "Hare 编程语言的官方风格指南中，行宽被限制为多少列？"
points = 20
hint = "格式：整数，如 120"
answers = ["80"]

[[questions]]
text = """在某次在线学术会议上，展示了一种通过声学侧信道推断 VoIP 呼叫来源的攻击手段，
请问这个会议的名称是？"""
points = 20
hint = "格式：会议名称 + 年份，以空格分割，如 ECOOP 2024"
answers = ["ACM WiSec 2021", "WiSec 2021"]

[flag]
env = "FLAG"
file = "/flag"
static_str = "flag{neko_quiz_static_flag}"

[message]
incorrect = "没有全部答对，不能给你 FLAG 哦。"
correct = "🎉🎉🎉 $FLAG 🎉🎉🎉"
```

| 配置项      | 子项           | 说明                                             | 示例                                                                                   |
| ----------- | -------------- | ------------------------------------------------ | -------------------------------------------------------------------------------------- |
| `general`   | -              | 通用配置项                                       | -                                                                                      |
|             | `title`        | 问答标题                                         | `"猫咪问答"`                                                                           |
|             | `return_score` | 是否返回分数                                     | `true`                                                                                 |
| `questions` | -              | 题目列表，可包含多个问题                         | -                                                                                      |
|             | `text`         | 问题正文，支持 HTML 标签                         | `"想要在苏州图书馆借阅 <i>Engineering a Compiler</i>，需要到哪个分馆的哪个馆藏地点？"` |
|             | `points`       | 问题分值                                         | `20`                                                                                   |
|             | `hint`         | 答题提示，支持 HTML 标签                         | `"格式：所在分馆-所在馆藏地点，例如 中心馆-西文书库。（如有多个，任意一个即可）"`      |
|             | `answers`      | 正确答案列表，支持多个答案                       | `[ "苏图-北馆书库", "苏图-设计图书馆" ]`                                               |
| `flag`      | -              | Flag 获取方式配置                                | -                                                                                      |
|             | `env`          | 从环境变量获取 Flag                              | `"FLAG"`                                                                               |
|             | `file`         | 从文件获取 Flag                                  | `"/flag"`                                                                              |
|             | `static_str`   | 静态字符串作为 Flag                              | `"flag{neko_quiz_static_flag}"`                                                        |
| `message`   | -              | 返回消息配置                                     | -                                                                                      |
|             | `incorrect`    | 答题未全部正确时的提示消息                       | `"没有全部答对，不能给你 FLAG 哦。"`                                                   |
|             | `correct`      | 答题全部正确时的提示消息，`$FLAG` 为 Flag 占位符 | `"🎉🎉🎉 $FLAG 🎉🎉🎉"`                                                                |

### Flag 获取优先级

系统按以下优先级获取 Flag：

1. 环境变量（`env`）
2. 文件（`file`）
3. 静态字符串（`static_str`）

### 注意事项

1. 题目文本字段（`text`、`hint`）支持 HTML 标签
2. 提示信息（`message`）不支持 HTML 标签，因为错误信息可能受到用户控制
3. `answers` 数组支持多个正确答案，用户答对其中任意一个即视为正确
4. `return_score` 设置为 `true` 时会在响应中返回用户得分
5. Flag 占位符 `$FLAG` 会在答题全部正确时被替换为实际的 Flag 值

## 部署

NekoQuiz 支持多种部署方式，默认端口为 `3000`。可以根据您的需求选择合适的部署方法。

运行参数通过环境变量进行配置，详细说明可见[帮助](#%E5%B8%AE%E5%8A%A9)。

### 预构建二进制文件

如果从是 [Releases](https://github.com/13m0n4de/neko-quiz/releases) 下载的压缩包，解压所有文件到同一目录并运行 `neko-quiz` 即可。

1. 解压下载的压缩包：
    ```
    tar xvf x86_64-unknown-linux-musl.tar.gz
    ```
2. 运行可执行文件：
    ```
    ./neko-quiz
    ```
    指定地址和端口：
    ```
    LEPTOS_SITE_ADDR="0.0.0.0:8080" ./neko-quiz
    ```
    指定配置文件：
    ```
    QUIZ_CONFIG="./my_config.toml" ./neko-quiz
    ```

### Docker 镜像

确保挂载的配置文件 `config.toml` 路径正确。

- 使用环境变量作为 Flag：
    ```
    docker run -d --rm -p 3000:3000 \
        -v ./config.toml:/app/config.toml \
        -e FLAG='flag{example}' \
        --name neko-quiz ghcr.io/13m0n4de/neko-quiz
    ```
- 使用文件提供 Flag：
    ```
    docker run -d --rm -p 3000:3000 \
        -v ./config.toml:/app/config.toml \
        -v ./flag:/flag \
        --name neko-quiz ghcr.io/13m0n4de/neko-quiz
    ```
- 使用 docker-compose，编辑 docker-compose.yml 文件配置环境变量和文件挂载，然后运行：
    ```
    docker-compose up -d
    ```

### 本地开发部署

- 开发模式（代码修改自动重新编译和热重载）：
    ```
    cargo leptos watch
    ```
- 构建 Release 版本：
    ```
    cargo leptos build --release
    ```
    构建完成后运行：
    ```
    ./target/bin-release/neko-quiz
    ```
- 或构建本地 Docker 镜像：
    ```
    docker build . -t neko-quiz
    ```
    ```
    docker run -d --rm -p 3000:3000 \
        -v ./config.toml:/app/config.toml \
        -e FLAG='flag{example}' neko-quiz
    ```

## 帮助

### 环境变量配置

可以通过以下环境变量来配置运行参数：

| 环境变量           | 默认值          | 描述               |
| ------------------ | --------------- | ------------------ |
| `RUST_LOG`         | `info`          | 设置日志级别       |
| `LEPTOS_SITE_ADDR` | `0.0.0.0:3000`  | 设置监听地址和端口 |
| `LEPTOS_SITE_ROOT` | `./site`        | 指定站点根目录     |
| `QUIZ_CONFIG`      | `./config.toml` | 指定配置文件路径   |

更多 Leptos 相关环境变量，可以参考 [cargo-leptos 对环境变量的说明](https://github.com/leptos-rs/cargo-leptos#environment-variables)。

## 使用案例

- [SVUCTF/SVUCTF-SPRING-2024 猫咪问答](https://github.com/SVUCTF/SVUCTF-SPRING-2024/tree/main/challenges/misc/neko_quiz)
- [SVUCTF/SVUCTF-WINTER-2023 猫娘问答](https://github.com/SVUCTF/SVUCTF-WINTER-2023/tree/main/challenges/misc/neko_quiz)

## 许可证

该项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解更多细节。

## 相关项目

- [USTC-Hackergame 猫咪小测](https://github.com/USTC-Hackergame/hackergame2023-writeups/blob/master/official/%E7%8C%AB%E5%92%AA%E5%B0%8F%E6%B5%8B/README.md)
