# 玄机 · Xuanji

民俗文化娱乐小站：**今日黄历、今日一签、答案之书、星座运势**四件套。
历法核心由 Rust 编写并编译为 WebAssembly 在浏览器中运行，计算结果与经典黄历库
（6tail 的 lunar 系列）逐项核对一致。

在线体验：<https://xiaoyu-hue.github.io/xuanji/>

## 四件套

- **今日黄历**：农历日期、干支、生肖、每日宜忌、值神、星宿、冲煞、
  财神/喜神/福神方位、彭祖百忌、节气。
- **今日一签**：20 首原创签诗配原创解签；以日期为种子，同一天同一签。
- **答案之书**：40 条原创答案，心里想好问题，翻开即答。
- **星座运势**：十二星座每日运势（整体/爱情/事业/财运、幸运色、幸运数字），
  同星座同日结果固定。

## 技术结构

| 目录 | 内容 |
|---|---|
| `core/` | Rust 核心库：黄历（基于 lunar_rust）、抽签、答案、运势生成 |
| `web/` | Rust → WebAssembly 绑定层 |
| `content/` | 签诗库与答案库（JSON，均为原创） |
| `index.html` + `pkg/` | 页面与编译产物（GitHub Pages 根目录） |

本地构建：

```sh
cd core && cargo run --release   # 命令行演示，四件套各输出一份
```

## 许可

- **源代码**：[Mozilla Public License 2.0](LICENSE)（MPL-2.0）
- **文档**（本 README 及仓库内说明文档）：CC BY-SA 4.0
- **历法地基**：[lunar_rust](https://github.com/flameoftheforest/lunar_rust)
  （MIT License），其为 6tail 的 lunar 系列（MIT License）的 Rust 移植；
  相关版权声明归其原作者所有。
- 签诗与答案文本为本项目原创内容，随源代码许可一同提供。

## 定位

民俗文化娱乐项目，仅供消遣，不作任何决策依据；不提供收费占卜服务。
