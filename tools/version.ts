import * as glob from "glob";
import * as fs from "fs/promises";

(async () => {
    // get all package.json files excluding node_modules
    const packageJsonPaths = glob.sync("**/package.json", {
        cwd: process.cwd(),
        ignore: ["**/node_modules/**"],
    });

    // update all version numbers to last argument
    const version = process.argv[process.argv.length - 1];
    for (const packageJsonPath of packageJsonPaths) {
        const data = await fs.readFile(packageJsonPath, "utf8").then(data => JSON.parse(data));
        data["version"] = version;

        await fs.writeFile(packageJsonPath, JSON.stringify(data, null, 4));
    }
})().then();
