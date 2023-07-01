const fs = require('fs');

const filePath = "/home/salaah/json-lineage/sample_data/sample.json";

const stream = fs.createReadStream(filePath, 'utf8');

stream.on('data', (chunk: string) => {
  const json: object = JSON.parse(chunk);
  console.log(json);
});

stream.on('error', (err: Error) => {
  console.error('Error reading file:', err);
});