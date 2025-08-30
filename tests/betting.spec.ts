import * as anchor from '@coral-xyz/anchor'
import { Program } from '@coral-xyz/anchor'
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js'
import { Betting } from '../target/types/betting'
import { createMint, getOrCreateAssociatedTokenAccount, mintTo } from '@solana/spl-token'

describe('betting', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)

  const connection = provider.connection;
  const payer = provider.wallet as anchor.Wallet;

  const program = anchor.workspace.Betting as Program<Betting>;

  let mint: PublicKey;

  const receiver = new anchor.Wallet(payer.payer)

  const ROUND_ID = Math.floor(Date.now() / 1000);

  // it('Create Treasury', async () => {

  //   await program.methods
  //     .createTreasury(
  //   )
  //     .accounts({
  //     })
  //     .rpc()

  //   const [treasuryInfo] = PublicKey.findProgramAddressSync(
  //     [Buffer.from("treasury")],
  //     program.programId
  //   )

  //   const treasuryInfoData = await program.account.treasury.fetch(treasuryInfo);

  //   console.log("Starting round:");
  //   console.table({
  //     "Treasury Address": treasuryInfoData.treasurer.toBase58(),
  //   });

  //   expect(treasuryInfoData.treasurer).toEqual(new PublicKey("i2tZJMMTqrcYv53qdLFsouL1JQPWgKiTfZ6sRDfk7nL"));
  // })

  it('Start round', async () => {

    await program.methods
      .startRound(
        new anchor.BN(ROUND_ID),
      )
      .accounts({
        priceUpdate: '7UVimffxr9ow1uXYxsr4LHAcV58mLzhmwaeKvJ1pjLiE'
      })
      .rpc()

    const [roundInfo] = PublicKey.findProgramAddressSync(
      [Buffer.from("round"), new Uint8Array(new anchor.BN(ROUND_ID).toArray("le", 8))],
      program.programId
    )

    const roundInfoData = await program.account.round.fetch(roundInfo);

    console.log("Starting round:");
    console.table({
      "Price Open": roundInfoData.priceOpen.toNumber(),
    });

    expect(roundInfoData.settled).toEqual(false);
  })




})
