import {createRequire} from "module";

const require = createRequire(import.meta.url);

const addon = require("../index.node");

console.log(addon.helloWorld());