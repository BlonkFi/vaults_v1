import * as anchor from "@coral-xyz/anchor";
import { Program, AnchorError } from "@coral-xyz/anchor";
import {
  PublicKey,
  Keypair,
  SendTransactionError,
  Transaction,
} from "@solana/web3.js";
import { getMint } from "@solana/spl-token";
import { BlonkfiVaults } from "../target/types/blonkfi_vaults";
import { assert } from "chai";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  getAccount,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import * as bs58 from "bs58";
import { generateFundedKeypair, createLocalhostConnection } from "./utils";

const connection = createLocalhostConnection();

describe("BlonkfiVaults", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.BlonkfiVaults as Program<BlonkfiVaults>;

  let blonkFiPlaceholderAdmin: Keypair;
  let slot: number;
  let centralVaultAddress: PublicKey;

  before(async () => {
    blonkFiPlaceholderAdmin = await generateFundedKeypair(connection);
    slot = await connection.getSlot();

    const seeds = [Buffer.from("BlonkFiCentralVault")];

    const centralVault = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    )[0];

    await program.methods
      .initCentralVault()
      .accounts({
        centralVault: centralVault,
        authority: blonkFiPlaceholderAdmin.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([blonkFiPlaceholderAdmin])
      .rpc();

    centralVaultAddress = centralVault;

    //Access the state of the central vault and check if the authority is set correctly
    const centralVaultAccount = await program.account.centralVault.fetch(
      centralVault
    );

    assert.equal(
      centralVaultAccount.authority.toBase58(),
      blonkFiPlaceholderAdmin.publicKey.toBase58()
    );
  });

  //Should fail if the central vault is re-initialized
  it("Fails to re-initialize the central vault", async () => {
    const seeds = [Buffer.from("BlonkFiCentralVault")];

    const centralVault = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    )[0];

    //Create a lookup table
    const [lookupTableInst, lookupTableAddress] =
      anchor.web3.AddressLookupTableProgram.createLookupTable({
        authority: blonkFiPlaceholderAdmin.publicKey,
        payer: blonkFiPlaceholderAdmin.publicKey,
        recentSlot: slot,
      });

    try {
      await program.methods
        .initCentralVault()
        .accounts({
          centralVault: centralVault,
          authority: blonkFiPlaceholderAdmin.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([blonkFiPlaceholderAdmin])
        .rpc();
    } catch (error) {
      const sendTransactionError = error as SendTransactionError;
      assert.include(sendTransactionError.message, "0x0");
    }
  });

  it("Creates a BONK vault", async () => {
    //BONK address from Mainnet
    const bonkMint = new PublicKey(
      "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263"
    );

    const bonkVaultSeeds = [
      Buffer.from("BlonkFiIndividualVault"),
      bonkMint.toBuffer(),
    ];

    const bonkVault = anchor.web3.PublicKey.findProgramAddressSync(
      bonkVaultSeeds,
      program.programId
    )[0];

    const receiptMintSeeds = [
      Buffer.from("BlonkFiReceiptMint"),
      bonkVault.toBuffer(),
    ];

    const receiptMint = anchor.web3.PublicKey.findProgramAddressSync(
      receiptMintSeeds,
      program.programId
    )[0];

    const centralVault = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("BlonkFiCentralVault")],
      program.programId
    )[0];

    try {
      await program.methods
        .createVault(new anchor.BN(1728446899))
        .accounts({
          authority: blonkFiPlaceholderAdmin.publicKey,
          vault: bonkVault,
          receiptMint: receiptMint,
          centralVault: centralVault,
          assetMint: bonkMint,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        })
        .signers([blonkFiPlaceholderAdmin])
        .rpc();

      //assert that the individual vault has the correct state
      const individualVault = await program.account.individualVault.fetch(
        bonkVault
      );
      assert.equal(individualVault.assetMint.toBase58(), bonkMint.toBase58());
    } catch (error) {
      const sendTransactionError = error as SendTransactionError;
      console.log("error is", sendTransactionError);
    }
  });

  it("Creates a WIF vault", async () => {
    //WIF address from Mainnet
    const wifMint = new PublicKey(
      "EKpQGSJtjMFqKZ9KQanSqYXRcF8fBopzLHYxdM65zcjm"
    );

    const wifVaultSeeds = [
      Buffer.from("BlonkFiIndividualVault"),
      wifMint.toBuffer(),
    ];

    const wifVault = anchor.web3.PublicKey.findProgramAddressSync(
      wifVaultSeeds,
      program.programId
    )[0];

    const receiptMintSeeds = [
      Buffer.from("BlonkFiReceiptMint"),
      wifVault.toBuffer(),
    ];

    const receiptMint = anchor.web3.PublicKey.findProgramAddressSync(
      receiptMintSeeds,
      program.programId
    )[0];

    const centralVault = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("BlonkFiCentralVault")],
      program.programId
    )[0];

    try {
      await program.methods
        .createVault(new anchor.BN(1728446899))
        .accounts({
          authority: blonkFiPlaceholderAdmin.publicKey,
          vault: wifVault,
          receiptMint: receiptMint,
          centralVault: centralVault,
          assetMint: wifMint,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        })
        .signers([blonkFiPlaceholderAdmin])
        .rpc();

      //assert that the individual vault has the correct state
      const individualVault = await program.account.individualVault.fetch(
        wifVault
      );
      assert.equal(individualVault.assetMint.toBase58(), wifMint.toBase58());
    } catch (error) {
      const sendTransactionError = error as SendTransactionError;
      console.log("error is", sendTransactionError);
    }
  });

  it("Creates a MOTHER vault", async () => {
    //MOTHER address from Mainnet
    const motherMint = new PublicKey(
      "3S8qX1MsMqRbiwKg2cQyx7nis1oHMgaCuc9c4VfvVdPN"
    );

    const motherVaultSeeds = [
      Buffer.from("BlonkFiIndividualVault"),
      motherMint.toBuffer(),
    ];

    const motherVault = anchor.web3.PublicKey.findProgramAddressSync(
      motherVaultSeeds,
      program.programId
    )[0];

    const receiptMintSeeds = [
      Buffer.from("BlonkFiReceiptMint"),
      motherVault.toBuffer(),
    ];

    const receiptMint = anchor.web3.PublicKey.findProgramAddressSync(
      receiptMintSeeds,
      program.programId
    )[0];

    const centralVault = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("BlonkFiCentralVault")],
      program.programId
    )[0];

    try {
      await program.methods
        .createVault(new anchor.BN(1728446899))
        .accounts({
          authority: blonkFiPlaceholderAdmin.publicKey,
          vault: motherVault,
          receiptMint: receiptMint,
          centralVault: centralVault,
          assetMint: motherMint,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        })
        .signers([blonkFiPlaceholderAdmin])
        .rpc();

      //assert that the individual vault has the correct state
      const individualVault = await program.account.individualVault.fetch(
        motherVault
      );
      assert.equal(individualVault.assetMint.toBase58(), motherMint.toBase58());
    } catch (error) {
      const sendTransactionError = error as SendTransactionError;
      console.log("error is", sendTransactionError);
    }
  });
});
