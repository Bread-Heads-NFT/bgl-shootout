const path = require("path");
const { generateIdl } = require("@metaplex-foundation/shank-js");

const idlDir = path.join(__dirname, "..", "idls");
const binaryInstallDir = path.join(__dirname, "..", ".crates");
const programDir = path.join(__dirname, "..", "programs");

generateIdl({
  generator: "shank",
  programName: "bgl_shootout",
  programId: "DUCKdJdA2Hexw9ZS7M5rcDfJneVF3ptQF1NDEJ9RMTbj",
  idlDir,
  binaryInstallDir,
  programDir: path.join(programDir, "bgl-shootout"),
});
