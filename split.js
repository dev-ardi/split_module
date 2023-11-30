const fs = require("fs");

const performance = require("perf_hooks").performance;
let t0 = performance.now()
console.log(t0);

const input = process.argv[2];
const path = process.argv[3];
let json = JSON.parse(fs.readFileSync(input, "utf8"));

const arr = eval(`json.${path}`);
if (!arr) throw "can't find it";

let ret = [];
for (let x of arr) {
	let add = JSON.parse(JSON.stringify(json));
	add[path] = [x];
	ret.push(add);
}

let count = 1;
for (let x of ret) {
	fs.writeFileSync(`output${count === 1 ? "" : count}.json`, JSON.stringify(x, 2));
	count++;
}

let t1 = performance.now() - t0;
console.error("done in: " + t1 + "ms")
