// Here we export some useful types and functions for interacting with the Anchor program.
import { AnchorProvider, Program } from '@coral-xyz/anchor'
import { Cluster, PublicKey } from '@solana/web3.js'
import BettingIDL from '../target/idl/betting.json'
import type { Betting } from '../target/types/betting'

// Re-export the generated IDL and type
export { Betting, BettingIDL }

// The programId is imported from the program IDL.
export const BETTING_PROGRAM_ID = new PublicKey(BettingIDL.address)

// This is a helper function to get the Tokenbetting Anchor program.
export function getBettingProgram(provider: AnchorProvider) {
  return new Program(BettingIDL as Betting, provider)
}

// This is a helper function to get the program ID for the Tokenbetting program depending on the cluster.
export function getBettingProgramId(cluster: Cluster) {
  switch (cluster) {
    case 'devnet':
    case 'testnet':
      // This is the program ID for the Tokenbetting program on devnet and testnet.
      return new PublicKey('CNd5sdD29D2Vip3d2qhzhimd2wXDDqSgpQS2vjP6u9W6')
    case 'mainnet-beta':
    default:
      return BETTING_PROGRAM_ID
  }
}
