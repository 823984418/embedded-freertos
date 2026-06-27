# embedded-freertos

为Rust提供便捷易用的FreeRTOS集成

## 指定 `FreeRTOSConfig.h`

通过设置环境变量以设置配置文件

```toml
[env]
EMBEDDED_FREERTOS_INCLUDE = { value = "包含FreeRTOSConfig.h的目录", relative = true }
```
