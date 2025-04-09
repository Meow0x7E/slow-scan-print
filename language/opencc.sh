#!/usr/bin/bash

# 运行这个以可以简单的生成对应的 `zh-HK` 和 `zh-TW` 翻译

cd "$(dirname $0)"

input='zh-CN.yml'

for locale in "hk" "tw"; do
    output="zh-${locale^^}.yml"
    config="/usr/share/opencc/s2${locale}.json"

    opencc -i "$input" -o "$output" -c "$config"
    sed -i -e '1i\# 该文件是使用 OpenCC 从 '"$input"' 转换的，请不要直接编辑它' "$output"
done