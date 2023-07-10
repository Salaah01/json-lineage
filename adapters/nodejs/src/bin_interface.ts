const { spawn } = require("child_process");
const path = require("path");
import { Readable } from "stream";

const get_bin_path = () => {
  const binDir = path.join(__dirname, "..", "bin");
  if (process.platform === "win32") {
    return path.join(binDir, "win32-x64", "jsonl_converter.exe");
  } else {
    return path.join(binDir, "other", "jsonl_converter");
  }
};

class BinReader {
  private readonly _binPath: string;
  private readonly _filePath: string;
  private readonly _messy: boolean;
  private iterator: AsyncIterableIterator<string>;

  private readonly _childProcess: any;

  constructor(binPath: string, filePath: string, messy: boolean = false) {
    this._binPath = binPath;
    this._filePath = filePath;
    this._messy = messy;
    this._childProcess = spawn(this._binPath, [this._filePath]);
    this.iterator = this.generateIterator();
  }

  private binArgs = (): string[] => {
    return [this._filePath, this._messy ? "--messy" : ""];
  };

  spawnChildProcess = (): any => {
    return spawn(this._binPath, this.binArgs());
  };

  private async *generateIterator(): AsyncGenerator<string> {
    const readable = this.spawnChildProcess().stdout;
    const reader = readable[Symbol.asyncIterator]();

    while (true) {
      const { done, value } = await reader.next();
      if (done) break;
      // setTimeout(() => {
      //   null; // Do nothing
      // }, Math.floor(Math.random() * (3 - 1 + 1) + 1));

      yield value.toString().trim();
    }
  }

  async next(): Promise<IteratorResult<string>> {
    return this.iterator.next();
  }

  [Symbol.asyncIterator](): AsyncIterableIterator<string> {
    return this;
  }
}

// const BIN = get_bin_path();
// const childProcess = spawn(BIN, [
//   "/home/salaah/json-lineage/sample_data/100kb_ssample.json",
// ]);

// childProcess.stdout.on("data", (data: string) => {
//   // Process the output from the binary
//   console.log(`Output: ${data}`);
// });

// // Handle any errors that occur during execution
// childProcess.on("error", (error: any) => {
//   console.error(`Error executing ${BIN}: ${error.message}`);
// });

// // Handle the completion of the binary execution
// childProcess.on("close", (code: number) => {
//   if (code === 0) {
//     console.log(`${BIN} executed successfully.`);
//   } else {
//     console.error(`${BIN} execution failed with code ${code}.`);
//   }
// });

const FP = "/home/salaah/json-lineage/sample_data/32mb_sample.json";

const binReader = new BinReader(get_bin_path(), FP);

// for (const line of binReader) {
//   console.log(line);
// }


(async () => {
  for await (const line of binReader) {
    console.log(line);
  }
}
)();