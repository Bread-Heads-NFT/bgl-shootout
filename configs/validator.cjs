const path = require("path");

const programDir = path.join(__dirname, "..", "programs");
function getProgram(dir, programName) {
  return path.join(programDir, dir, "target", "deploy", programName);
}

module.exports = {
  validator: {
    commitment: "processed",
    programs: [
      {
        label: "Bgl Shootout",
        programId: "DUCKdJdA2Hexw9ZS7M5rcDfJneVF3ptQF1NDEJ9RMTbj",
        deployPath: getProgram("bgl-shootout", "bgl_shootout.so"),
      },
    ],
  },
};
