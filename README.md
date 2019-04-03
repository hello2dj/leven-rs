# leven-rs

> 参照 [leven](https://github.com/sindresorhus/leven) 实现，使用 neon & rust 绑定 node 实现  [Levenshtein distance](https://en.wikipedia.org/wiki/Levenshtein_distance)

# 使用前提
需要先安装 [Rust](https://www.rust-lang.org) 以及 [neon](https://neon-bindings.com/docs/getting-started)

# 安装

```
$ git clone 

$ cd leven-rs

$ npm run install
```

# 用例

```
const leven = require('.');

leven('cat', 'cow');
// => 2
```

# Benchmark

```
$ npm run bench
```

```
230,410 op/s » leven
224,650 op/s » talisman
  3,388 op/s » leven-rs
  1,046 op/s » levenshtein-edit-distance
    725 op/s » fast-levenshtein
    581 op/s » levenshtein-component
    152 op/s » ld
    246 op/s » levenshtein
    168 op/s » levdist
      7 op/s » natural
```

可看出 leven-rs 在速度上并不突出，数据的传递 js -> v8 -> neon -> rust 是个很大的问题，可以看出并不是用 addon，就会加速，数据传递的损耗也很大，只有计算的代价远远大于数据传递代价时，使用 addon 才会有意义。

> 数据传递的消耗非常大