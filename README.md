
# Haversinejs
Javascript Haversine Distance Calculator

## Installation

Use the package manager [npm](https://www.npmjs.com/) to install haversinecalc.

```bash
npm i haversinecalc
```

## Usage

```javascript
const {Point, haversine} = require("haversinecalc")

const src_p = new Point(-7.0,7.0) // source (lat,lon)
const dst_p = new Point(-8.0,8.0) // destinantion (lat,lon)

haversine(src_p, dst_p, is_miles) // is_miles bool
```

## Benchmark
```bash
manual js : 156581.10278754708
manual: 7.526ms

using wasm : 156581.10278754708
wasm: 0.311ms
```

### Thankyou~~~