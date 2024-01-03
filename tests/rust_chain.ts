import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RustChain } from "../target/types/rust_chain";

describe("rust_chain", () => {
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  const program = anchor.workspace.RustChain as Program<RustChain>;
  const myAccount = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    // Create a new account for UserProfile

    // Send a transaction to initialize the UserProfile
    const tx = await program.methods.initialize().accounts({
      userProfile: myAccount.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([ myAccount]) // Include userProfileAccount as a signer
      .rpc();

    console.log("Your transaction signature", tx);
  });
});

