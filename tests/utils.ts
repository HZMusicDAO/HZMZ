import { PublicKey } from "@solana/web3.js";
import * as anchor from "@project-serum/anchor";
import { HZMZ_PROGRAM_ID } from "./constants";

const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

async function getPDAPublicKey(seeds: Array<Buffer | Uint8Array>, programId: PublicKey) {
  return (await getPDA(seeds, programId))[0];
}

function getPDA(seeds: Array<Buffer | Uint8Array>, programId: PublicKey) {
  return PublicKey.findProgramAddressSync(
    seeds,
    programId
);
}

const getMetadata = async (mint: anchor.web3.PublicKey): Promise<anchor.web3.PublicKey> => {
  return await getPDAPublicKey(
    [Buffer.from("metadata"), TOKEN_METADATA_PROGRAM_ID.toBuffer(), mint.toBuffer()],
    TOKEN_METADATA_PROGRAM_ID,
  );
};

const getMasterEdition = async (mint: anchor.web3.PublicKey): Promise<anchor.web3.PublicKey> => {
  return await getPDAPublicKey(
    [Buffer.from("metadata"), TOKEN_METADATA_PROGRAM_ID.toBuffer(), mint.toBuffer(), Buffer.from("edition")],
    TOKEN_METADATA_PROGRAM_ID,
  );
};


const getUserData = async (mintAuthority: anchor.web3.PublicKey): Promise<anchor.web3.PublicKey> => {
  return await getPDAPublicKey([Buffer.from("userdata"), mintAuthority.toBuffer()], HZMZ_PROGRAM_ID);
};


const findPlayerDataAcc = (player: PublicKey) =>{
  const [listingDataAcc, bump] = PublicKey.findProgramAddressSync(
      [player.toBuffer(), Buffer.from("hzmz")],
      HZMZ_PROGRAM_ID
  );    

  return listingDataAcc
}

const findGameDataAcc = () => {
  const [assetManager] = PublicKey.findProgramAddressSync(
      [Buffer.from("hzmz")],
      HZMZ_PROGRAM_ID
  );
  return assetManager
}


const findGameTreasuryAcc = ()=> {
  const [assetManager, bump] = PublicKey.findProgramAddressSync(
      [Buffer.from("hzmz_wallet")],
      HZMZ_PROGRAM_ID
  );
  return assetManager
}


// Types

type TFinalized = { authority: PublicKey };
type TInitialized = { owner: PublicKey, totalSupply: Number, seedBalance: Number, mrfWallet: PublicKey, bkWallet: PublicKey, shiragaWallet: PublicKey };
type TNFTMinted = { nftNum: Number, holder: PublicKey, uri: String };
type TFeeUpdated = { fee: Number };
type TDevWithdrawn = { owner: PublicKey, devBalance: Number; mrfAmount: Number, bkAmount: Number, shiragaAmount:Number, authority: PublicKey };
type TUserWithdrawn = { owner: PublicKey, amount: Number; lastInteraction: Number, sellTotal: Number };
type TSell= { owner: PublicKey, sellTotal: Number, nftHolder:Boolean, lastInteraction: Number };

export { TOKEN_METADATA_PROGRAM_ID,
  findPlayerDataAcc, findGameDataAcc, findGameTreasuryAcc, getUserData,
  getPDAPublicKey, TDevWithdrawn,  TFinalized, TInitialized, TNFTMinted, TFeeUpdated, TUserWithdrawn, TSell, 
  getMetadata, getMasterEdition };
