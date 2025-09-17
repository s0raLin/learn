# 运行 Rust 学习项目中的示例

本项目包含多个按功能分类的模块，每个模块中都有独立的可执行示例。

## 运行命令

现在所有示例都已经在 `Cargo.toml` 中配置为二进制目标，可以直接使用 cargo run 命令运行：

```bash
cargo run --bin <二进制目标名称>
```

例如：

```bash
# 运行 trait 继承示例
cargo run --bin trait_inheritance

# 运行泛型结构体示例
cargo run --bin generic_struct

# 运行 Option/Result 处理示例
cargo run --bin option_result_handling
```

## 各模块示例及对应的二进制目标名称

### 1. basic 模块（基础示例）
- `box_memory_allocation` - Box 内存分配示例
- `comprehensive_rust_examples` - 综合 Rust 示例
- `expression_tree_evaluation` - 表达式树求值示例
- `medium_example_5` 到 `medium_example_57` - 各种中级示例
- `large_example_8` 等 - 大型示例
- `small_example_9` 等 - 小型示例

### 2. collections 模块（集合类型）
- `counter_implementation` - 计数器实现示例

### 3. enums 模块（枚举类型）
- `enum_memory_layout` - 枚举内存布局示例

### 4. generics 模块（泛型）
- `generic_function` - 泛型函数示例
- `generic_struct` - 泛型结构体示例

### 5. option_result 模块（Option/Result类型）
- `match_option_example` - Option 匹配示例
- `option_result_handling` - Option/Result 处理示例

### 6. strings 模块（字符串操作）
- `string_pop_operations` - 字符串 pop 操作示例

### 7. structs 模块（结构体）
- `struct_impl_examples` - 结构体实现示例

### 8. traits 模块（Trait）
- `trait_inheritance` - Trait 继承示例

## 构建所有示例

构建所有已配置的示例：

```bash
cargo build
```

或者构建所有目标：

```bash
cargo build --all-targets
```

## 查看所有可用的二进制目标

查看项目中所有可用的二进制目标：

```bash
cargo run --bin
```

## 直接编译和运行单个文件

对于学习目的，也可以直接使用 rustc 编译单个文件：

```bash
# 进入特定模块目录
cd src/bin/traits

# 编译
rustc trait_inheritance.rs

# 运行
./trait_inheritance
```

这种方式不需要在 `Cargo.toml` 中配置二进制目标。