import { UmiPlugin } from '@metaplex-foundation/umi';
import { createBglShootoutProgram } from './generated';

export const bglShootout = (): UmiPlugin => ({
  install(umi) {
    umi.programs.add(createBglShootoutProgram(), false);
  },
});
