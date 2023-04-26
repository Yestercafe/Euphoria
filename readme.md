# Euphoria

Euphoria UE C++ docs generator

## 相关

使用了 Euphoria 的项目：

- [Lazarus-glhf/WhatCubes](https://github.com/Lazarus-glhf/WhatCubes)

## GitHub Actions

使用 GitHub Actions 持续集成 Euphoria 文档可参考：<https://github.com/Lazarus-glhf/WhatCubes/blob/master/.github/workflows/docs-ci.yml>

## To-do List

目前在使用 DFA 解析文件，但是没有对抽象出自动机。之后引入嵌套类后可能要改用带状态的自动机，这时候视情况要不要把封装起来（其实是取决于我想不想）。

- [x] Parser 功能可用
  - [x] 支持类成员
  - [x] 支持方法
  - [ ] 支持类内部枚举类型
  - [ ] 支持成员的访问限制分析
  - [ ] 支持内部类
  - [ ] 分析变量类型、默认值
  - [ ] 分析方法返回值类型
- [ ] 生成 Markdown 功能可用
- [x] 生成 HTML 功能可用

## 使用

目前 Euphoria 的解析力较弱，对注释格式有强要求。

目前仅支持对 UE C++ 要求格式的 `.h` 文件进行分析。

在使用 UE 要求的 C++ 格式的基础上，在成员函数的定义（在 `.cpp` 文件中）上面定义 Euphoria 文档注释。

格式可以参考：

```cpp
/**
 * @desc
 * 计算点到点的距离。
 * Euphoria 文档注释以 /** 开头，注意这一行除了这 3 个字符以外不要有任何字符。
 * 中间的每一行均需要以「空格星号空格」开头。
 * 开头是 @ 的是文档的字段，现在支持 desc/description、param/parameter、returns/return 三种。
 * 注意比如写了 @desc 之后从下一行开始才是第一行描述。
 *
 * 可以随意空行，但是目前会吃掉所有的空行
 *
 * @param rhs
 * 另一个点。
 * 参数名写在 @param 同行的后面。
 * 注意到 @param 下面也是可以写多行注释的。
 * 
 * @returns
 * 两点距离（double）。
 * 返回值同样支持多行的说明。
 * 目前没有额外支持表示返回值类型，如果需要可以写在描述里。
 *
 * 注释以「空格星号斜线」结尾，下一行必须紧接函数定义。
 */
double UPoint::Distance(UPoint* rhs)
{
    return std::hypot(this->x - rhs->x, this->y - rhs->y);
}
```
