/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  AccountMeta,
  Context,
  PublicKey,
  Serializer,
  Signer,
  TransactionBuilder,
  mapSerializer,
  transactionBuilder,
} from '@metaplex-foundation/umi';
import { addObjectProperty, isWritable } from '../shared';

// Accounts.
export type StartGameInstructionAccounts = {
  /** The PDA of the game */
  gamePda: PublicKey;
  /** The account paying for the storage fees */
  payer?: Signer;
  /** The system program */
  systemProgram?: PublicKey;
  /** The authority who will control gameplay */
  authority?: PublicKey;
};

// Data.
export type StartGameInstructionData = {
  discriminator: number;
  matchName: string;
  mint: PublicKey;
  numRounds: number;
};

export type StartGameInstructionDataArgs = {
  matchName: string;
  mint: PublicKey;
  numRounds: number;
};

export function getStartGameInstructionDataSerializer(
  context: Pick<Context, 'serializer'>
): Serializer<StartGameInstructionDataArgs, StartGameInstructionData> {
  const s = context.serializer;
  return mapSerializer<
    StartGameInstructionDataArgs,
    any,
    StartGameInstructionData
  >(
    s.struct<StartGameInstructionData>(
      [
        ['discriminator', s.u8()],
        ['matchName', s.string()],
        ['mint', s.publicKey()],
        ['numRounds', s.u8()],
      ],
      { description: 'StartGameInstructionData' }
    ),
    (value) => ({ ...value, discriminator: 0 })
  ) as Serializer<StartGameInstructionDataArgs, StartGameInstructionData>;
}

// Args.
export type StartGameInstructionArgs = StartGameInstructionDataArgs;

// Instruction.
export function startGame(
  context: Pick<Context, 'serializer' | 'programs' | 'payer'>,
  input: StartGameInstructionAccounts & StartGameInstructionArgs
): TransactionBuilder {
  const signers: Signer[] = [];
  const keys: AccountMeta[] = [];

  // Program ID.
  const programId = {
    ...context.programs.getPublicKey(
      'bglShootout',
      'DUCKdJdA2Hexw9ZS7M5rcDfJneVF3ptQF1NDEJ9RMTbj'
    ),
    isWritable: false,
  };

  // Resolved inputs.
  const resolvingAccounts = {};
  const resolvingArgs = {};
  addObjectProperty(resolvingAccounts, 'payer', input.payer ?? context.payer);
  addObjectProperty(
    resolvingAccounts,
    'systemProgram',
    input.systemProgram ?? {
      ...context.programs.getPublicKey(
        'splSystem',
        '11111111111111111111111111111111'
      ),
      isWritable: false,
    }
  );
  const resolvedAccounts = { ...input, ...resolvingAccounts };
  const resolvedArgs = { ...input, ...resolvingArgs };

  // Game Pda.
  keys.push({
    pubkey: resolvedAccounts.gamePda,
    isSigner: false,
    isWritable: isWritable(resolvedAccounts.gamePda, true),
  });

  // Payer.
  signers.push(resolvedAccounts.payer);
  keys.push({
    pubkey: resolvedAccounts.payer.publicKey,
    isSigner: true,
    isWritable: isWritable(resolvedAccounts.payer, true),
  });

  // System Program.
  keys.push({
    pubkey: resolvedAccounts.systemProgram,
    isSigner: false,
    isWritable: isWritable(resolvedAccounts.systemProgram, false),
  });

  // Authority (optional).
  if (resolvedAccounts.authority) {
    keys.push({
      pubkey: resolvedAccounts.authority,
      isSigner: false,
      isWritable: isWritable(resolvedAccounts.authority, false),
    });
  }

  // Data.
  const data =
    getStartGameInstructionDataSerializer(context).serialize(resolvedArgs);

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}
