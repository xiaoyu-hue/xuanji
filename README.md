# 玄机 · Xuanji

东方传统民俗文化娱乐小站：**今日黄历（含时辰吉凶）、今日一签、答案之书、万事择日、八字排盘、称骨、合婚、犯太岁、解梦、紫微斗数**，东方民俗十件套。
历法核心由 Rust 编写并编译为 WebAssembly，全部计算在浏览器本地完成；
历法结果与经典黄历库逐项核对一致。

[![License: MPL-2.0](https://img.shields.io/badge/License-MPL--2.0-brightgreen.svg)](LICENSE)
[![Docs: CC BY-SA 4.0](https://img.shields.io/badge/Docs-CC%20BY--SA%204.0-lightgrey.svg)](LICENSE-DOCS)
[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-orange.svg)](https://www.rust-lang.org/)
[![在线体验](https://img.shields.io/badge/在线体验-xiaoyu--hue.github.io-blue.svg)](https://xiaoyu-hue.github.io/xuanji/)

**🌐 在线体验：<https://xiaoyu-hue.github.io/xuanji/>**（手机、电脑均可，无需注册与安装）

## 功能一览

| 功能 | 说明 |
|---|---|
| 今日黄历 | 农历、干支、生肖、每日宜忌、值神、星宿、冲煞、财神/喜神/福神方位、彭祖百忌、节气 |
| 任意日期查询 | 黄历可查询 1900–2100 年任意一天，一键回到今天 |
| 今日一签 | 100 首签诗配原创解签（以原创为主，个别化用古典诗词）；以日期为种子，同一天同一签 |
| 答案之书 | 100 条原创答案，心里想好问题，翻开即答 |
| 分享图 | 黄历图、签文图在浏览器本地生成，可下载或长按保存 |
| 八字排盘 | 输入出生日期、时间与性别，排出四柱（干支、藏干、十神、纳音、长生）、五行分布、胎元/命宫/身宫与八步大运 |
| 万事择日 | 按区间筛选「宜」某事项（嫁娶/出行/移徙/开市等 18 种）的日子，可避开冲本人生肖的日期 |
| 犯太岁 | 按年份与生肖查值 / 冲 / 刑 / 害 / 破，附全年犯太岁名单 |
| 称骨 | 袁天罡称骨歌：年/月/日/时四项骨重相加，对照 51 首传统歌诀（通行本） |
| 合婚 | 生肖配对、纳音五行、日主天干三项民间简法合参，给出契合分数与说明 |
| 解梦 | 116 条本站原创梦境词条，可按关键词搜索、按分类浏览 |
| 紫微斗数 | 完整十二宫命盘：十四主星与主要辅星、庙旺亮度、四化、大限；安星与 iztro 逐盘核对 |

玩法细节与常见问题见 [使用说明](docs/使用说明.md)。

## 快速开始

**使用者**：直接打开上面的在线地址即可，全部功能免费。

**开发者**：

```sh
rustup target add wasm32-unknown-unknown
cd core && cargo run --release   # 核心自检：命令行演示四件套
cd ../web && cargo build --release --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/xuanji_web.wasm \
  --out-dir ../pkg --target web   # 生成页面用的 JS/WASM 包
```

架构、验证方法、内容库格式与发布流程见 [开发文档](docs/开发文档.md)。

## 项目结构

| 路径 | 内容 |
|---|---|
| `core/` | Rust 核心库（每体系一个模块文件 + 共用层，见架构文档） |
| `web/` | Rust → WebAssembly 绑定层（每体系一个绑定文件，JSON 接口） |
| `content/` | 内容库：签诗、答案、解梦词条（JSON，按体系分文件） |
| `index.html`、`app.js`、`js/` + `pkg/` | 页面（编排器 + 每功能一个模块）与编译产物（GitHub Pages 根目录） |
| `docs/` | 使用说明、开发文档、架构文档 |

## 依赖与开源致谢

玄机站在以下开源工作的肩膀上，特此致谢：

| 依赖 | 用途 | 许可 | 链接 |
|---|---|---|---|
| lunar_rust | 历法计算地基（农历、干支、宜忌等） | MIT | [flameoftheforest/lunar_rust](https://github.com/flameoftheforest/lunar_rust) |
| lunar 系列（6tail） | lunar_rust 的上游原作，本项目的验算金标准 | MIT | [6tail/lunar-javascript](https://github.com/6tail/lunar-javascript) |
| Rust 语言与标准库 | 核心与绑定层开发语言 | MIT / Apache-2.0 | [rust-lang.org](https://www.rust-lang.org/) |
| wasm-bindgen | Rust ↔ WebAssembly 绑定生成 | MIT / Apache-2.0 | [rustwasm/wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) |
| GitHub Pages | 站点托管 | — | [pages.github.com](https://pages.github.com/) |

其中 lunar_rust 与 lunar 系列的版权归其原作者所有，依 MIT 许可使用；
本项目的历法输出已与 lunar-javascript 对多个日期逐项对照验证。

## 定位与路线图

玄机的终局定位是**三馆全图**的民俗文化娱乐小站（5.0.0 拍板）：
东方馆做全，西方馆、心智馆逐馆回归并补齐，答案之书归西方馆。
当前进度：

- **已上线（东方馆十件套）**：今日黄历（含时辰吉凶）、今日一签、答案之书、
  万事择日、八字排盘、称骨、合婚、犯太岁、解梦、紫微斗数
- **进行中**：5.0.0 已完成模块化架构重构（分层、低耦合、故障隔离，
  见架构文档）；东方馆继续补齐（八字神煞、紫微流年、周易、梅花、六爻、
  九宫飞星、奇门遁甲）→ 西方馆 → 心智馆

历史注记：3.0.0 曾收窄为东方专馆并下线星座运势、今日塔罗、16 型人格，
相关代码与内容保留在仓库历史中，将按三馆路线在新架构上恢复。

全部内容坚持原创写作（个别签诗化用古典除外，见更新日志 2.0.2），
全部计算在本机完成，不收集任何数据。

## 文档

- [使用说明](docs/使用说明.md)：各功能玩法与常见问题
- [架构文档](docs/架构文档.md)：分层与模块划分、依赖规则、验算体系、扩展流程
- [开发文档](docs/开发文档.md)：构建、验证、内容库格式与发布流程
- [更新日志](CHANGELOG.md)

## 许可证

- **源代码**：[Mozilla Public License 2.0](LICENSE)（MPL-2.0）
- **文档**（README、docs/、CHANGELOG 等）：[CC BY-SA 4.0](LICENSE-DOCS)
- **第三方依赖**：各依其自身许可（见上表），版权归原作者所有
- 签诗、答案与解签文本以本项目原创为主，个别签诗化用古典诗词与
  传统谚语（均为公有领域作品），整体随源代码许可一同提供

## 免责声明

- 玄机是**民俗文化娱乐**项目：黄历部分为真实历法计算；签文、运势与
  答案之书属于娱乐内容，**不构成任何建议，请勿据此作出决策**。
- 本站**不提供收费占卜服务**，全部功能免费；任何以“玄机”名义收费
  占卜、售卖“改运消灾”服务的，均与本项目无关。
- 本站**不收集任何数据**：全部计算在访问者的浏览器本地完成，
  不上传、不存储个人信息，也没有账号系统。
- 历法数据经与经典黄历库逐项核对，但不对因使用本站信息产生的
  任何后果承担责任。

## 反馈

发现问题或有新点子，欢迎在仓库的
[Issues](https://github.com/xiaoyu-hue/xuanji/issues) 里留言。
