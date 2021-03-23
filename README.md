# Auto Body Temperature

Rust製の"**超**"高速体温生成。

## Benchmark

```bash
cargo bench

Gnuplot not found, using plotters backend
auto temp               time:   [29.282 ns 29.404 ns 29.531 ns]
```

## Usage

- `cargo.toml`に以下を追加してください。

```toml
[dependencies]
auto_temp = { git = "https://github.com/yuto51942/auto-body-temperature", branch="main" }
```

```rust
use auto_temp::Temp;

fn main() -> Result<(), Box<dyn std::error:Error>> {
  let t = Temp::new(Some(36.0), None);

  // if one generate.
  let temp = t.create();
  println!("{:.1}", temp);

  // if multiple generate.
  let temps = t.create_multiple(10);
  for element in temps {
    println!("{:.1}", element);
  }
}
```

## Function Description

### `Temp`

#### `new`

Tempインスタンスを作成します。

##### Args

- average (Option<f32>): 平均値。Noneの場合は36.0になります。
- sigma (Option<f32>): 正規分布の分散。Noneの場合は1.0になります。

###### Returns

- self

#### `create`

1つの体温を生成します。

##### Rerurns

- u32: 体温。newで指定した最大最小平均をもとに正規分布を作成しランダムに選択します。

#### `create_multiple`

複数の体温を生成します。

##### Args

- size: 何個の体温を生成するか。

##### Returns

Vec[u32]: 体温のリスト。

## LICENSE

[MIT](./LICENSE)
