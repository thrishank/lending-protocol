import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BN } from "bn.js";
import { Lending } from "../target/types/lending";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

describe("lending", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Lending as Program<Lending>;

  const provider = anchor.getProvider();

  const connection = provider.connection;

  const user = anchor.web3.Keypair.generate();
  const bank = anchor.web3.Keypair.generate();

  const confirm = async (signature: string): Promise<string> => {
    const block = await connection.getLatestBlockhash();
    await connection.confirmTransaction({
      signature,
      ...block,
    });
    return signature;
  };

  const log = async (signature: string): Promise<string> => {
    console.log(
      `Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}`
    );
    return signature;
  };

  it("airdrop", async () => {
    const tx1 = await connection.requestAirdrop(user.publicKey, 1_000_000_000);
    await confirm(tx1);
    const tx2 = await connection.requestAirdrop(bank.publicKey, 10_000_000_000); // 1 SOL
    await confirm(tx2);
  });

  let mint: anchor.web3.PublicKey;
  let mint2: anchor.web3.PublicKey;
  it("create mint", async () => {
    mint = await createMint(connection, user, user.publicKey, null, 9);
    const tokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      user,
      mint,
      user.publicKey
    );
    await mintTo(connection, user, mint, tokenAccount.address, user, 10000000);
    mint2 = await createMint(connection, user, user.publicKey, null, 9);
  });

  it("init user", async () => {
    await program.methods
      .initalizeUser()
      .accounts({
        signer: user.publicKey,
      })
      .signers([user])
      .rpc({ commitment: "confirmed" });
  });

  it("init bank", async () => {
    await program.methods
      .initalizeBank(new BN(1), new BN(1))
      .accounts({
        signer: bank.publicKey,
        mint: mint,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([bank])
      .rpc({ commitment: "confirmed" });
  });

  it("init bank 2", async () => {
    await program.methods
      .initalizeBank(new BN(1), new BN(1))
      .accounts({
        signer: bank.publicKey,
        mint: mint2,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([bank])
      .rpc({ commitment: "confirmed" });
  });
  it("user deposit", async () => {
    await program.methods
      .userDeposit(new BN(100))
      .accounts({
        user: user.publicKey,
        mint: mint,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([user])
      .rpc({ commitment: "confirmed" });
  });
  it("user withdraw", async () => {
    await program.methods
      .userWithdraw(new BN(1))
      .accounts({ user: user.publicKey, mint, tokenProgram: TOKEN_PROGRAM_ID })
      .signers([user])
      .rpc({ commitment: "confirmed" });
  });
});
