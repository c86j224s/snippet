import { WSAEMFILE } from "constants";

import("../pkg/index.js").catch(console.error).then(function(loaded) {
    loaded.greet("rustwasm/rust-webpack-template")
})
