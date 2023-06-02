const path = require("path");
const k = require("@metaplex-foundation/kinobi");

// Paths.
const clientDir = path.join(__dirname, "..", "clients");
const idlDir = path.join(__dirname, "..", "idls");

// Instanciate Kinobi.
const kinobi = k.createFromIdls([path.join(idlDir, "bgl_shootout.json")]);

// Update accounts.
kinobi.update(
  new k.UpdateAccountsVisitor({
    GameAccount: {
      seeds: [
        k.stringConstantSeed("game"),
        k.stringSeed("matchName", "The name of the match."),
        k.publicKeySeed("payerAddress", "The address of the payer and authority."),
        k.publicKeySeed("mint", "The mint address of the player."),
      ],
    },
    // ...
  })
);

// Update instructions.
kinobi.update(
  new k.UpdateInstructionsVisitor({
    create: {
      bytesCreatedOnChain: k.bytesFromAccount("GameAccount"),
    },
    // ...
  })
);

// Set ShankAccount discriminator.
const key = (name) => ({ field: "key", value: k.vEnum("Key", name) });
kinobi.update(
  new k.SetAccountDiscriminatorFromFieldVisitor({
    GameAccount: key("GameAccount"),
  })
);

// Render JavaScript.
const jsDir = path.join(clientDir, "js", "src", "generated");
const prettier = require(path.join(clientDir, "js", ".prettierrc.json"));
kinobi.accept(new k.RenderJavaScriptVisitor(jsDir, { prettier }));
