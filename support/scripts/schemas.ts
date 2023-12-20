import { quicktype, InputData, JSONSchemaInput, FetchingJSONSchemaStore } from "quicktype-core";

import fs from "fs";
import path from "path";

async function* walk(dir: string): AsyncIterable<string> {
    for await (const d of await fs.promises.opendir(dir)) {
        const entry = path.join(dir, d.name);
        if (d.isDirectory()) {
            yield* walk(entry);
        } else if (d.isFile()) {
            yield entry;
        }
    }
}

async function main() {
    const schemaInput = new JSONSchemaInput(new FetchingJSONSchemaStore());

    const filenames: string[] = [];
    for await (const p of walk("./support/schemas")) {
        filenames.push(p);
    }

    filenames.sort();

    for (const f of filenames) {
        const buffer = fs.readFileSync(f);
        const relative = path.relative(path.join(process.cwd(), "support/schemas"), f);
        await schemaInput.addSource({ name: relative, schema: buffer.toString() });
    }

    const inputData = new InputData();
    inputData.addInput(schemaInput);

    const ts = await quicktype({
        inputData,
        lang: "typescript",
        rendererOptions: {}
    });
    writeToFile("./languages/node/src/infisical_client/schemas.ts", ts.lines);

    const python = await quicktype({
        inputData,
        lang: "python",
        rendererOptions: {
            "python-version": "3.7"
        }
    });
    writeToFile("./crates/infisical-py/infisical_client/schemas.py", python.lines);

    /*
    const csharp = await quicktype({
        inputData,
        lang: "csharp",
        rendererOptions: {
            namespace: "Infisical.Sdk",
            framework: "SystemTextJson",
            "csharp-version": "6"
        }
    });
    writeToFile("./languages/csharp/Infisical.Sdk/schemas.cs", csharp.lines);

    const java = await quicktypeMultiFile({
        inputData,
        lang: "java",
        rendererOptions: {
            package: "com.infisical.sdk.schema",
            "java-version": "8"
        }
    });

    const javaDir = "./languages/java/src/main/java/com/infisical/sdk/schema/";
    if (!fs.existsSync(javaDir)) {
        fs.mkdirSync(javaDir);
    }
    java.forEach((file, path) => writeToFile(javaDir + path, file.lines));
    */
}

main();
function writeToFile(filename: string, lines: string[]) {
    const output = fs.createWriteStream(filename);
    lines.forEach(line => {
        output.write(line + "\n");
    });
    output.close();
}
