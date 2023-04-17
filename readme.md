# Euphoria
Euphoria UE C++ docs generator

## 使用

目前 Euphoria 的解析力较弱，对注释格式有强要求。

目前仅支持对 `.cpp` 文件的文档生成，`.h` 的稍候。

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
