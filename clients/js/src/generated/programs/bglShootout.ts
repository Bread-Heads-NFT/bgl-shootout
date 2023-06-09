/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  ClusterFilter,
  Context,
  Program,
  PublicKey,
  publicKey,
} from '@metaplex-foundation/umi';
import {
  getBglShootoutErrorFromCode,
  getBglShootoutErrorFromName,
} from '../errors';

export const BGL_SHOOTOUT_PROGRAM_ID = publicKey(
  'DUCKdJdA2Hexw9ZS7M5rcDfJneVF3ptQF1NDEJ9RMTbj'
);

export function createBglShootoutProgram(): Program {
  return {
    name: 'bglShootout',
    publicKey: BGL_SHOOTOUT_PROGRAM_ID,
    getErrorFromCode(code: number, cause?: Error) {
      return getBglShootoutErrorFromCode(code, this, cause);
    },
    getErrorFromName(name: string, cause?: Error) {
      return getBglShootoutErrorFromName(name, this, cause);
    },
    isOnCluster() {
      return true;
    },
  };
}

export function getBglShootoutProgram<T extends Program = Program>(
  context: Pick<Context, 'programs'>,
  clusterFilter?: ClusterFilter
): T {
  return context.programs.get<T>('bglShootout', clusterFilter);
}

export function getBglShootoutProgramId(
  context: Pick<Context, 'programs'>,
  clusterFilter?: ClusterFilter
): PublicKey {
  return context.programs.getPublicKey(
    'bglShootout',
    BGL_SHOOTOUT_PROGRAM_ID,
    clusterFilter
  );
}
