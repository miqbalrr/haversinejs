
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

const src_p = new Point(-7,7) // source (lat,lon)
const dst_p = new Point(-8,8) // destinantion (lat,lon)

haversine(src_p, dst_p, is_miles) // is_miles bool

Multiple count
===================================================
const {haversines} = require("haversinecalc")

let src1 = {
   lat: 7,
   lon: 7
}

let src2 = {
   lat:6,
   lon:6
}

let dst = {
   lat: 5,
   lon: 5
}

let points = [
   {
      src: src1,
      dst,
      is_miles: false
   },
   {
      src: src2,
      dst,
      is_miles: false
   }
]
console.log(haversines(points))

Float64Array(2) [ 313638.2117169125, 156890.78148558066 ]
```

## Benchmark
```bash
manual js : 156581.10278754708
manual: 7.526ms

using wasm : 156581.10278754708
wasm: 0.311ms
```

### Thankyou~~~