---
name: decode.rs 追加基础单元测试
overview: 在 `src/protocol/decode.rs` 末尾追加一个 `#[cfg(test)] mod tests`，用最基础的 `assert!` / `assert_eq!` / `println!` 验证三类反馈帧解码；只挑文档中当前实现能正常解码的示例，保证 `cargo test` 通过；不改动任何现有代码、不引入复杂语法。
todos:
  - id: add-status-tests
    content: 在 decode.rs 末尾追加测试模块并添加状态帧解码测试
    status: completed
  - id: add-fault-param-tests
    content: 在同一模块补充故障详情帧与参数响应帧解码测试
    status: completed
    dependencies:
      - add-status-tests
  - id: run-cargo-test
    content: 运行 cargo test 查看输出并确认测试全部通过
    status: completed
    dependencies:
      - add-status-tests
      - add-fault-param-tests
---

## 产品概述
依据 `FlowDrive_CAN_反馈帧全集.md` 中的示例帧，在 `src/protocol/decode.rs` 文件最下方追加一组基础测试函数，用于验证状态帧、故障详情帧、参数事务响应帧三类反馈帧的解码结果。

## 核心特性
- 追加一个内联测试模块，用最基础的测试函数、断言和打印语句验证解码值，可查看每次解码的输出
- 仅选用文档中当前实现能正常解码的示例帧，保证测试正常运行、全部通过
- 不改动现有解码逻辑、常量与其他文件，仅在目标文件末尾追加内容，保持与项目现有测试写法一致
- 本次不运行 `cargo test`，只交付测试代码

## 技术栈
沿用项目现有 Rust（edition 2024）与标准库内置测试机制，只使用 `#[cfg(test)]`、`#[test]`、`assert!`、`assert_eq!`、`println!` 等基础语法，不引入任何新依赖，不查阅第三方包源码，不做额外调研。

## 实现方式
在 `src/protocol/decode.rs` 末尾追加单个 `#[cfg(test)] mod tests`，用 `use super::*;` 引入三个解码函数。三个结果结构体的字段都是私有的，因此测试必须内联在该文件内，才能直接读取字段、打印与断言。完全照搬 `encode.rs` 与 `command.rs` 文件末尾的测试组织方式，不新增抽象、不新增辅助函数、不改动上方任何生产代码。

## 关键执行说明
- 解码函数返回 Option，测试里用 `unwrap()` 取出结构体；枚举字段用 `as u8` / `as u16` 转成整数（每个字段只转换一次并存到局部变量）再断言或打印，位标志字段用 `bits()`，浮点字段用简单断言（负值给一个很小的容差），全程只用基础语法。
- 只选文档里不会触发 panic 的帧：状态帧的 IDLE、开环运行、故障态、负数、模式对比等 8 字节示例；故障详情帧的相过流、硬件 VDS_OCP、通讯失联、母线过压、I²t 加堵转全部实例；参数响应帧只选错误码非成功且操作码在 0x80 至 0x85、参数 ID 在枚举内的示例，以及 COMMIT / SAVE / RESET 成功且负载全 0 的示例。
- 必须避开的帧：成功后读回非零值（0.1f、600、0xFFFFFFFF、0.2f 等）、未知参数 ID 0x0999、操作码 0x86 与 0xFF、长度不足 8 字节、非法枚举字节；这些在当前实现里会直接 panic，放进测试会导致失败。
- 影响范围控制：仅追加测试模块，不回退用户已改的 `panic!` 逻辑，不改 `constants.rs`，不改其它文件。

## 架构设计
单文件内追加：测试模块 → 调用同文件的三个解码函数 → 读取私有字段 → 打印并断言。无新模块、无跨文件依赖，结构非常简单，无需架构图。

## 目录结构
```
flow_driver/
└── src/protocol/
    └── decode.rs   # [MODIFY] 仅在文件最末尾追加 #[cfg(test)] mod tests：状态帧、故障详情帧、参数响应帧三类解码的基础测试函数，用 assert! 与 println! 输出并验证解码结果；上方现有代码完全不动。
```

## 交付说明
按用户要求不执行 `cargo test`，写完测试代码后直接交付，不做编译或测试验证。