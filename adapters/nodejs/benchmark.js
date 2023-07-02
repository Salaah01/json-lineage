const fs = require("fs");
const _process = require("process");

const FILE_PATH = "/home/salaah/json-lineage/sample_data/500kb_sample.json";

const withStream = () => {
  const stream = fs.createReadStream(FILE_PATH, "utf8");

  stream.on("data", (chunk) => {
    JSON.parse(chunk);
  });

  stream.on("error", (err) => {
    console.error("Error reading file:", err);
  });
};

const withReadFile = () => {
  fs.readFile(FILE_PATH, "utf8", (err, data) => {
    if (err) {
      console.error("Error reading file:", err);
      return;
    }

    JSON.parse(data);
  });
};


const benchmark = (fn) => {
  const start = performance.now();
  const mem_start = _process.memoryUsage().heapUsed;
  fn();
  const end = performance.now();
  const mem_end = _process.memoryUsage().heapUsed;
  const mem_used_mb = (mem_end - mem_start) / 1024 / 1024;

  console.log(`${fn.name}: ${end - start}ms, ${mem_used_mb}MB`);
};

const main = () => {
//   benchmark(withStream);
  benchmark(withReadFile);
};

main();
