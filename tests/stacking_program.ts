import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { StackingProgram } from "../target/types/stacking_program";
import wallet from "../Admin-wallet.json";
import { Commitment, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { createMint, getAssociatedTokenAddress, getAssociatedTokenAddressSync, getOrCreateAssociatedTokenAccount, mintTo, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { assert } from "chai";
import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

describe("stacking_program", () => {

  const commitment: Commitment = "confirmed";
  // Helper function to log a message  
    const log = async (signature: string): Promise<string> => {
      console.log(
        `Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}\n`
      );
      return signature;
    };
  
    const confirmTx = async (signature: string) => {
      const latestBlockhash = await anchor.getProvider().connection.getLatestBlockhash();
      await anchor.getProvider().connection.confirmTransaction(
        {
          signature,
          ...latestBlockhash,
        },
        commitment
      )
    }
  
    const confirmTxs = async (signatures: string[]) => {
      await Promise.all(signatures.map(confirmTx))
    }

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.stackingProgram as Program<StackingProgram>;

  const provider = anchor.getProvider();
  const connection = provider.connection;

  const admin = Keypair.fromSecretKey(new Uint8Array(wallet));

  const config = PublicKey.findProgramAddressSync(
    [Buffer.from("config")],
    program.programId
  )[0];

  const reward_mint = PublicKey.findProgramAddressSync(
    [Buffer.from("rewards"), config.toBuffer()],
    program.programId
  )[0];

  console.log("reward_mint : ", reward_mint.toBase58());

  const user = Keypair.generate();
  
  let mint: PublicKey;

  it("Airdrop and create Mints",async ()=>{

    await Promise.all([admin,user].map(async (k) => {
      return await anchor.getProvider().connection.requestAirdrop(k.publicKey, 100 * anchor.web3.LAMPORTS_PER_SOL)
    })).then(confirmTxs);

    mint = await createMint(
      connection,
      user,
      user.publicKey,
      user.publicKey,
      6
    );

  })


  it("Is initialized config", async () => {
    
    const tx = await program.methods
    .initializeConfig(
      200,
      100,
      50,
      0,
    )
    .accountsStrict({
      admin: admin.publicKey,
      config: config,
      rewardMint: reward_mint,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId
    })
    .signers([admin])
    .rpc();

    console.log("Your transaction signature", tx);
  });

  let user_account: PublicKey;
  let user_reward_ata: PublicKey;
  let stake_account: PublicKey;

  it("Init User", async ()=>{
    user_account = PublicKey.findProgramAddressSync(
      [Buffer.from("user"), user.publicKey.toBuffer()],
      program.programId
    )[0];

    console.log("user_account :", user_account);

    const tx = await program.methods
    .initializeUser()
    .accountsStrict({
      user: user.publicKey,
      userAccount: user_account,
      systemProgram: anchor.web3.SystemProgram.programId
    })
    .signers([user])
    .rpc()

    console.log("tx:", tx);
  })

  it("stake sol", async ()=>{

    user_reward_ata = (await getOrCreateAssociatedTokenAccount(
      connection,
      user,
      reward_mint,
      user.publicKey
    )).address;

    const reward_recieved_initial = await connection.getTokenAccountBalance(user_reward_ata);

    console.log("rewards_received initial:", reward_recieved_initial);

    stake_account = PublicKey.findProgramAddressSync(
      [Buffer.from("stake"), config.toBuffer(), user.publicKey.toBuffer()],
      program.programId
    )[0];

    const vault = PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), stake_account.toBuffer()],
      program.programId
    )[0];

    const tx = await program.methods
    .stakeSol(new anchor.BN(1_000_000_000))
    .accountsStrict({
      user: user.publicKey,
      rewardMint: reward_mint,
      userRewardAta: user_reward_ata,
      stakeAccount: stake_account,
      config: config,
      vault: vault,
      userAccount: user_account,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId
    })
    .signers([user])
    .rpc()

    console.log("tx :", tx);

    const vault_balance = await connection.getBalance(vault);
    console.log("vault_balace :", vault_balance);
    assert(vault_balance === 1*LAMPORTS_PER_SOL, "Vault Balance not equail");

    const reward_recieved = await connection.getTokenAccountBalance(user_reward_ata);

    console.log("rewards_received :", reward_recieved?.value?.uiAmount);

  })

  let mint_ata: PublicKey;
  let vault_ata: PublicKey;
  let stake_account_spl: PublicKey;

  it("stake spl token", async () => {

    mint_ata = (await getOrCreateAssociatedTokenAccount(
      connection,
      user,
      mint,
      user.publicKey,
    )).address;

    console.log("mint_ata :", mint_ata);

    await mintTo(
      connection,
      user,
      mint,
      mint_ata,
      user.publicKey,
      1000 * 1_000_000
    ).then(confirmTx);

    stake_account_spl = PublicKey.findProgramAddressSync(
      [Buffer.from("stake"), config.toBuffer(), user.publicKey.toBuffer(), mint.toBuffer()],
      program.programId
    )[0];


    vault_ata = getAssociatedTokenAddressSync(
      mint,
      stake_account_spl,
      true
    )

    console.log("vault_ata :", vault_ata);

    console.log(
      "user", user.publicKey,
      "mint", mint,
      "mintAta", mint_ata,
      "rewardMint", reward_mint,
      "userRewardAta", user_reward_ata,
      "stakeAccount", stake_account_spl,
      "config", config,
      "vaultAta", vault_ata,
      "userAccount", user_account,
    )

    const tx = await program.methods
    .stakeSpl(new anchor.BN(10_000_000))
    .accountsStrict({
      user: user.publicKey,
      mint: mint,
      mintAta: mint_ata,
      rewardMint: reward_mint,
      userRewardAta: user_reward_ata,
      stakeAccount: stake_account_spl,
      config: config,
      vaultAta: vault_ata,
      userAccount: user_account,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId
    })
    .signers([user])
    .rpc()

    console.log("tx :", tx);

    const vault_ata_balance = await connection.getTokenAccountBalance(vault_ata);
    console.log("vault_balace :", vault_ata_balance);
    assert(vault_ata_balance?.value?.uiAmount === 10, "Vaul_ata Balance not equail");

    const reward_recieved = await connection.getTokenAccountBalance(user_reward_ata);

    console.log("rewards_received :", reward_recieved?.value?.uiAmount);
  })
  
});
