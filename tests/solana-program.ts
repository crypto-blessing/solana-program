import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolanaProgram } from "../target/types/solana_program";

describe("solana-program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.SolanaProgram as Program<SolanaProgram>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
