# fs-playground

## 编译
```bash
mkdir build
cd build
# 默认为仿真模式，如需硬件模式，则调用 'cmake -DCMAKE_BUILD_TYPE=Prerelease ..'
cmake ..
make
```

## 运行示例程序
```bash
make example
```

## 现有问题

- 调用 `bufio_read` 函数不会 core dump，但调用 `read_file` 会 core dump，读取的文件大小为 11M，降低到 7.7MB 左右就不会 core dump