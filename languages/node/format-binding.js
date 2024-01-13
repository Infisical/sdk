// We need to replace the "binding.js" file with a custom one that replaces all occurrences of "@infisical/napi" with "@infisical/sdk"

// We create our bindings (and the binding.js file) in the infisical-napi crate, which has one fundamental flaw.
// The binding.js file will assume that the binding files are called @infisical/napi, which is not the case.
// We can't name the two packages the same, as that would cause a conflict.

// To resolve this issue, we format the binding.js file before publishing the package.

const fs = require("node:fs");
const path = require("node:path");

const filePath = path.join(__dirname, "binding.js");

// Read the file
fs.readFile(filePath, "utf8", function (err, data) {
    if (err) {
        return console.log(err);
    }
    // Replace all occurrences of @infisical/napi with @infisical/sdk
    var result = data.replace(/@infisical\/napi/g, "@infisical/sdk");

    // Re-write the file
    fs.writeFile(filePath, result, "utf8", function (err) {
        if (err) return console.log(err);
    });
});
