const fs = require("fs");
const _process = require("process");

const FILE_PATH = "/home/salaah/json-lineage/sample_data/324mb_sample.json";

const withStream = () => {
  const stream = fs.createReadStream(FILE_PATH, "utf8");

  stream.on("data", (chunk) => {
    let s = JSON.parse(chunk);
    console.log(s);
  });

  stream.on("error", (err) => {
    console.error("Error reading file:", err);
  });
};

const withReadFile = async () => {

  const data = await fs.promises.readFile(FILE_PATH, "utf8");
};


const benchmark = async (fn) => {
  const start = performance.now();
  const mem_start = _process.memoryUsage().heapUsed;
  await fn();
  const end = performance.now();
  const mem_end = _process.memoryUsage().heapUsed;
  const mem_used_mb = (mem_end - mem_start) / 1024 / 1024;

  console.log(`${fn.name}: ${(end - start).toFixed(2)} ms, ${mem_used_mb.toFixed(2)} MB`);
};

const main = () => {
  benchmark(withReadFile);
};

main();
