import * as anchor from "@project-serum/anchor";
import { Program, Wallet } from "@project-serum/anchor";
import {
  createAssociatedTokenAccountInstruction,
  createInitializeMintInstruction,
  getAssociatedTokenAddress,
  MINT_SIZE,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token"; // IGNORE THESE ERRORS IF ANY
import { SolanaNFT } from "../target/types/solana_nft";
import * as utils from "./utils";
import { BK_WALLET, HZMZ_PROGRAM_ID } from "./constants";
import { PublicKey } from "@solana/web3.js";
import { expect } from 'chai';

const { SystemProgram } = anchor.web3;

const programId = HZMZ_PROGRAM_ID;

describe("solana_nft_test", async () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const wallet = provider.wallet as Wallet;

  const program = anchor.workspace.SolanaNFT as Program<SolanaNFT>;
  const contractDataPublic = utils.getPDAPublicKey([Buffer.from("contractdata")], programId);
  const treasuryDataPublic = utils.getPDAPublicKey([Buffer.from("treasury")], programId);

  let collectionAuthorityRecord: PublicKey; 
  let collectionMint: PublicKey;
  let collectionMetadata: PublicKey;
  let collectionMasterEdition: PublicKey;
  let collectionUpdateAuthority: PublicKey;

  let gameStateAccount = await utils.findGameDataAcc();
  let gameTreasuryAccount = await utils.findGameTreasuryAcc();

  let test_NftTokenAccount: any;
  
  const handleFinalizedEvent = (ev: utils.TFinalized) =>
    console.log(`  ${program.idl.events[0].name} Event ==>`, {
      authority: ev.authority.toString(),
    });
  const handleInitializedEvent = (ev: utils.TInitialized) =>
    console.log(`  ${program.idl.events[1].name} Event ==>`, {
      totalSupply: ev.totalSupply.toString(),
      bkWallet: ev.bkWallet.toString(),
      seedBalance: ev.seedBalance.toString(),
      owner: ev.owner.toString(),
    });
  const handleNFTMintedEvent = (ev: utils.TNFTMinted) =>
    console.log(`  ${program.idl.events[2].name} Event ==>`, {
      nftNum: ev.nftNum.toString(),
      holder: ev.holder.toString(),
      uri: ev.uri.toString(),
    });
  const handleNFTCollectionMintedEvent = (ev: utils.TNFTMinted) =>
    console.log(`  ${program.idl.events[3].name} Event ==>`, {
      nftNum: ev.nftNum.toString(),
      holder: ev.holder.toString(),
      uri: ev.uri.toString(),
    });
  const handleFeeUpdatedEvent = (ev: utils.TFeeUpdated) =>
    console.log(`  ${program.idl?.events[4]?.name} Event ==>`, {
      fee: ev?.fee?.toString(),
    });  
  const handleDevWithdrawnEvent = (ev: utils.TDevWithdrawn) =>
  console.log(`  ${program.idl.events[5].name} Event ==>`, {
    devBalance: ev?.devBalance?.toString(),
    bkAmount: ev?.bkAmount?.toString(),
    authority: ev?.authority?.toString(),
  });
  const handleUserWithdrawnEvent = (ev: utils.TUserWithdrawn) =>
  console.log(`  ${program.idl.events[6].name} Event ==>`, {
    amount: ev?.amount?.toString(),
    lastInteraction: ev?.lastInteraction?.toString(),
    sellTotal: ev?.sellTotal?.toString(),
    owner: ev?.owner.toString(),
  });

  const devWithdrawListener = program.addEventListener(program.idl.events[0].name, handleDevWithdrawnEvent);
  const finalizedListener = program.addEventListener(program.idl.events[1].name, handleFinalizedEvent);
  const initializedListener = program.addEventListener(program.idl.events[2].name, handleInitializedEvent);
  const nftMintedListener = program.addEventListener(program.idl.events[3].name, handleNFTMintedEvent);
  const nftCollectionMintedListener = program.addEventListener(program.idl.events[4].name, handleNFTCollectionMintedEvent);
  const feeUpdatedListener = program.addEventListener(program.idl.events[5].name, handleFeeUpdatedEvent);
  const userWithdrawnListener = program.addEventListener(program.idl.events[6].name, handleUserWithdrawnEvent);

  it("Initialize", async () => {
    console.log("  contractDataPublic address ", (await contractDataPublic).toBase58());
    console.log("  treasuryDataPublic address ", (await treasuryDataPublic).toBase58());
    
    const tx = await program.methods
      .initialize(new anchor.BN(1e9), BK_WALLET, new anchor.BN(4444))
      .accounts({
        contractData: await contractDataPublic,
        treasury: await treasuryDataPublic,
        gameStateAccount: gameStateAccount,
        treasuryAccount: gameTreasuryAccount,
        authority: wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("  Your initialize transaction signature: ", tx);

  });
  
  it("Mint Collection NFT", async () => {

    let subdomainData = await utils.getPDAPublicKey([Buffer.from("test")], programId);

    const lamports: number = await program.provider.connection.getMinimumBalanceForRentExemption(MINT_SIZE);

    const mintKey: anchor.web3.Keypair = anchor.web3.Keypair.generate();
    test_NftTokenAccount = await getAssociatedTokenAddress(mintKey.publicKey, wallet.publicKey);

    const mint_tx = new anchor.web3.Transaction().add(
      anchor.web3.SystemProgram.createAccount({
        fromPubkey: wallet.publicKey,
        newAccountPubkey: mintKey.publicKey,
        space: MINT_SIZE,
        programId: TOKEN_PROGRAM_ID,
        lamports,
      }),
      createInitializeMintInstruction(mintKey.publicKey, 0, wallet.publicKey, wallet.publicKey),
      createAssociatedTokenAccountInstruction(wallet.publicKey, test_NftTokenAccount, wallet.publicKey, mintKey.publicKey),
    );

    const res = await program.provider.sendAndConfirm(mint_tx, [mintKey]);
    const metadataAddress: PublicKey = await utils.getMetadata(mintKey.publicKey);
    const masterEdition: PublicKey = await utils.getMasterEdition(mintKey.publicKey);

    const updateAuthorityPDA = await utils.getPDAPublicKey([Buffer.from("hzmz")], programId);

    collectionAuthorityRecord = updateAuthorityPDA;
    collectionMint = mintKey.publicKey;
    collectionMetadata = metadataAddress;
    collectionMasterEdition = masterEdition;
    collectionUpdateAuthority = updateAuthorityPDA;

    test_MetadataAddress = metadataAddress;
    console.log("  Account: ", res);
    console.log("  NFT Account: ", test_NftTokenAccount.toBase58());
    console.log("  Mint key: ", mintKey.publicKey.toString());
    console.log("  User: ", wallet.publicKey.toString());
    
    console.log("  Metadata address: ", metadataAddress.toBase58());
    console.log("  MasterEdition: ", masterEdition.toBase58());

    console.log("  collectionAuthorityRecord: ", collectionAuthorityRecord.toBase58());
    console.log("  collectionMint: ", collectionMint.toBase58());
    console.log("  collectionMetadata: ", collectionMetadata.toBase58());
    console.log("  collectionMasterEdition: ", collectionMasterEdition.toBase58());
    console.log("  collectionUpdateAuthority: ", collectionUpdateAuthority.toBase58());

    const uri = "  https://arweave.net/y5e5DJsiwH0s_ayfMwYk-SnrZtVZzHLQDSTZ5dNRUHA";

    const tx = await program.methods
      .mintNft(mintKey.publicKey, uri, "Hzmz", "hzmz")
      .accounts({
        mintAuthority: wallet.publicKey,
        mint: mintKey.publicKey,
        tokenAccount: test_NftTokenAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
        metadata: metadataAddress,
        tokenMetadataProgram: utils.TOKEN_METADATA_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        masterEdition: masterEdition,
        contractData: await contractDataPublic,
        userData: await utils.getUserData(wallet.publicKey),
        treasury: await treasuryDataPublic,
        subdomains: subdomainData,
      })
      .rpc();

    console.log("  Your mintNft transaction signature: ", tx);
  });

  it("Update fee", async () => {
    const tx = await program.methods
      .updateFee(new anchor.BN(123456789))
      .accounts({
        contractData: await contractDataPublic,
        authority: wallet.publicKey,
      })
      .rpc();

    console.log("  Your updateFee transaction signature: ", tx);
  });

  it("Mint NFT in Collection", async () => {
    const lamports = await program.provider.connection.getMinimumBalanceForRentExemption(MINT_SIZE);
      
    const mintKey = anchor.web3.Keypair.generate();
    const NftTokenAccount = await getAssociatedTokenAddress(mintKey.publicKey, wallet.publicKey);

    const metadataAddress = await utils.getMetadata(mintKey.publicKey);
    const masterEdition = await utils.getMasterEdition(mintKey.publicKey);

    let subdomainData = await utils.getPDAPublicKey([Buffer.from("hzmz")], programId);

    console.log("  collectionAuthorityRecord--->", collectionAuthorityRecord.toBase58())
    console.log("  NFT Account: ", NftTokenAccount.toBase58());
    console.log("  Mint key: ", mintKey.publicKey.toString());
    console.log("  User: ", wallet.publicKey.toString());
    console.log("  Metadata address: ", metadataAddress.toBase58());
    console.log("  MasterEdition: ", masterEdition.toBase58());
    console.log("  subdomainData: ", (await subdomainData).toBase58());

    const tx = await program.rpc.mintNftcollection(
        mintKey.publicKey,
        "https://arweave.net/9MY-M2zNET9rWKw0sn-MauUWQevFnPiVPcptreuC68Q",
        "Hzmz",
        "Hzmz",
        {
            accounts: {
              mintAuthority: wallet.publicKey,
              mint: mintKey.publicKey,
              tokenAccount: NftTokenAccount,
              tokenProgram: TOKEN_PROGRAM_ID,
              metadata: metadataAddress,//new PublicKey("BaKrmciZzdn9W75CGuyuhwhRxF4VAThQTXu8aWU7w6Kp"),
              tokenMetadataProgram: utils.TOKEN_METADATA_PROGRAM_ID,
              systemProgram: anchor.web3.SystemProgram.programId,
              rent: anchor.web3.SYSVAR_RENT_PUBKEY,
              masterEdition: masterEdition,
              contractData: await contractDataPublic,
              userData: await utils.getUserData(wallet.publicKey),
              treasury: await treasuryDataPublic,
              subdomains: await subdomainData,
              collectionAuthorityRecord: collectionAuthorityRecord,
              collectionMint: collectionMint,
              collectionMetadata: collectionMetadata,
              collectionMasterEdition: collectionMasterEdition,
              collectionUpdateAuthority: collectionAuthorityRecord,
            },
            signers: [mintKey],
            instructions: [
              anchor.web3.SystemProgram.createAccount({
                    fromPubkey: wallet.publicKey,
                    newAccountPubkey: mintKey.publicKey,
                    space: MINT_SIZE,
                    programId: TOKEN_PROGRAM_ID,
                    lamports,
              }),
              createInitializeMintInstruction(mintKey.publicKey, 0, wallet.publicKey, wallet.publicKey),
              createAssociatedTokenAccountInstruction(wallet.publicKey, NftTokenAccount, wallet.publicKey, mintKey.publicKey),
             ],
        }
    );

    console.log("  Your mintNft in Collection transaction signature: ", tx);
  });

  it("Set Collection for nft holder", async () => {
    
    const slot = await program.provider.connection.getSlot();
    const timestamp = await program.provider.connection.getBlockTime(slot);

    const tx = await program.methods
      .setCollection(collectionMint, new anchor.BN(timestamp))
      .accounts({
        gameAccount: await gameStateAccount,
        authority: wallet.publicKey,
      })
      .rpc();

    console.log("  Your setCollection transaction signature: ", tx);
  });

  it("dev withdraw", async () => {
    const tx = await program.methods
      .devWithdraw()
      .accounts({
        contractData: await contractDataPublic,
        treasury: gameTreasuryAccount,
        authority: wallet.publicKey,
        gameStateAccount: gameStateAccount,
        bkAccount: BK_WALLET,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("  Dev Withdraw transaction signature: ", tx);
  });

  it("Finalize", async () => {
    const tx = await program.methods
      .finalize()
      .accounts({
        contractData: await contractDataPublic,
        treasury: await treasuryDataPublic,
        authority: wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("  Your Finalize transaction signature: ", tx);
  });

  it("Remove event listeners", async function () {
    program.removeEventListener(finalizedListener);
    program.removeEventListener(initializedListener);
    program.removeEventListener(nftMintedListener);
    program.removeEventListener(nftCollectionMintedListener);
    program.removeEventListener(feeUpdatedListener);
    program.removeEventListener(userWithdrawnListener);
    program.removeEventListener(devWithdrawListener);
  });
});
