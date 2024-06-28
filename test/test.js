"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const rblasjs_js_1 = require("../pkg/rblasjs.js");
let a = new rblasjs_js_1.VectorF64(new Float64Array([1, 2, 3]));
console.log(a);
(0, rblasjs_js_1.print_cint)();
