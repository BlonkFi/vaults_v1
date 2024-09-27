import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, Keypair } from "@solana/web3.js";
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

describe("BlonkfiVaults", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.BlonkfiVaults as Program<BlonkfiVaults>;

  const blonkFiAdmin = Keypair.fromSecretKey(
    bs58.decode("YOUR_ADMIN_SECRET_KEY")
  );
  const blonkFiUser = Keypair.fromSecretKey(
    bs58.decode("YOUR_USER_SECRET_KEY")
  );

  let assetMint: PublicKey;
  let receiptMint: PublicKey;
  let centralVault: Keypair;
  let individualVault: Keypair;
  let userAssetAccount: PublicKey;
  let userReceiptAccount: PublicKey;
  let vaultAssetAccount: PublicKey;

  before(async () => {
    // Create asset mint
    assetMint = await createMint(
      program.provider.connection,
      blonkFiAdmin,
      blonkFiAdmin.publicKey,
      null,
      6
    );

    // Create central vault
    centralVault = Keypair.generate();

    // Initialize central vault
    await program.methods
      .initCentralVault(blonkFiAdmin.publicKey)
      .accounts({
        centralVault: centralVault.publicKey,
        authority: blonkFiAdmin.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([centralVault, blonkFiAdmin])
      .rpc();

    // Create user asset account
    userAssetAccount = (
      await getOrCreateAssociatedTokenAccount(
        program.provider.connection,
        blonkFiAdmin,
        assetMint,
        blonkFiUser.publicKey
      )
    ).address;

    // Mint some assets to the user
    await mintTo(
      program.provider.connection,
      blonkFiAdmin,
      assetMint,
      userAssetAccount,
      blonkFiAdmin,
      1000000000 // 1000 tokens with 6 decimals
    );
  });

  it("Initializes a central vault", async () => {
    const centralVaultAccount = await program.account.centralVault.fetch(
      centralVault.publicKey
    );
    assert.ok(centralVaultAccount.authority.equals(blonkFiAdmin.publicKey));
    assert.equal(centralVaultAccount.vaultAddresses.length, 0);
  });

  it("Initializes an individual vault", async () => {
    individualVault = Keypair.generate();
    receiptMint = Keypair.generate().publicKey;

    await program.methods
      .initIndividualVault(
        blonkFiAdmin.publicKey,
        centralVault.publicKey,
        individualVault.publicKey,
        new anchor.BN(86400) // 1 day lock period
      )
      .accounts({
        authority: blonkFiAdmin.publicKey,
        vault: individualVault.publicKey,
        receiptMint: receiptMint,
        assetMint: assetMint,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([individualVault, blonkFiAdmin])
      .rpc();

    const vaultAccount = await program.account.individualVault.fetch(
      individualVault.publicKey
    );
    assert.ok(vaultAccount.assetMint.equals(assetMint));
    assert.ok(vaultAccount.receiptMint.equals(receiptMint));
    assert.ok(vaultAccount.centralVaultAddress.equals(centralVault.publicKey));
  });

  it("Adds an individual vault to the central vault", async () => {
    await program.methods
      .addVault(blonkFiAdmin.publicKey, new anchor.BN(86400))
      .accounts({
        centralVault: centralVault.publicKey,
        authority: blonkFiAdmin.publicKey,
        newVault: individualVault.publicKey,
        receiptMint: receiptMint,
        assetMint: assetMint,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      })
      .signers([blonkFiAdmin])
      .rpc();

    const centralVaultAccount = await program.account.centralVault.fetch(
      centralVault.publicKey
    );
    assert.equal(centralVaultAccount.vaultAddresses.length, 1);
    assert.ok(
      centralVaultAccount.vaultAddresses[0].equals(individualVault.publicKey)
    );
  });

  it("Deposits tokens into the vault", async () => {
    vaultAssetAccount = (
      await getOrCreateAssociatedTokenAccount(
        program.provider.connection,
        blonkFiAdmin,
        assetMint,
        individualVault.publicKey
      )
    ).address;

    userReceiptAccount = (
      await getOrCreateAssociatedTokenAccount(
        program.provider.connection,
        blonkFiAdmin,
        receiptMint,
        blonkFiUser.publicKey
      )
    ).address;

    const depositAmount = new anchor.BN(100000000); // 100 tokens

    await program.methods
      .depositIntoVault(depositAmount)
      .accounts({
        depositor: blonkFiUser.publicKey,
        depositorTokenAccount: userAssetAccount,
        vault: individualVault.publicKey,
        vaultTokenAccount: vaultAssetAccount,
        receiptMint: receiptMint,
        depositorReceiptTokenAccount: userReceiptAccount,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .signers([blonkFiUser])
      .rpc();

    const vaultAccount = await program.account.individualVault.fetch(
      individualVault.publicKey
    );
    assert.equal(vaultAccount.totalAssets.toNumber(), 100000000);
    assert.equal(vaultAccount.totalShares.toNumber(), 100000000);

    const userReceiptBalance = await getAccount(
      program.provider.connection,
      userReceiptAccount
    );
    assert.equal(userReceiptBalance.amount.toString(), "100000000");
  });

  // it("Calculates total assets and shares", async () => {
  //   const [totalAssets] = await program.methods
  //     .calculateTotalAssets()
  //     .accounts({
  //       centralVault: centralVault.publicKey,
  //       vaultInfos: individualVault.publicKey,
  //     })
  //     .view();

  //   assert.equal(totalAssets.toNumber(), 100000000);

  //   const [totalShares] = await program.methods
  //     .calculateTotalShares()
  //     .accounts({
  //       centralVault: centralVault.publicKey,
  //       vaultInfos: individualVault.publicKey,
  //     })
  //     .view();

  //   assert.equal(totalShares.toNumber(), 100000000);
  // });

  it("Withdraws tokens from the vault", async () => {
    // Wait for the lock period to end
    await new Promise((resolve) => setTimeout(resolve, 1000));

    const withdrawAmount = new anchor.BN(50000000); // 50 tokens

    await program.methods
      .withdrawFromVault(withdrawAmount)
      .accounts({
        withdrawer: blonkFiUser.publicKey,
        depositorReceiptTokenAccount: userReceiptAccount,
        vault: individualVault.publicKey,
        vaultTokenAccount: vaultAssetAccount,
        receiptMint: receiptMint,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      })
      .signers([blonkFiUser])
      .rpc();

    const vaultAccount = await program.account.individualVault.fetch(
      individualVault.publicKey
    );
    assert.equal(vaultAccount.totalAssets.toNumber(), 50000000);
    assert.equal(vaultAccount.totalShares.toNumber(), 50000000);

    const userReceiptBalance = await getAccount(
      program.provider.connection,
      userReceiptAccount
    );
    assert.equal(userReceiptBalance.amount.toString(), "50000000");
  });
});
