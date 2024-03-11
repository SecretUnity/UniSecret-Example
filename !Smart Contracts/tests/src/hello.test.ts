import { expect } from "chai";
import { SecretNetworkClient, Wallet } from "secretjs";
import {
  ensureChainIsReady,
  getGenesisWallets,
  getLocalSecretConnection,
  requestFaucetForAddress,
  storeAndInitContract,
} from "./utils/localsecret";

describe("Basic tests", () => {
  let client: SecretNetworkClient;
  let wallet: Wallet;

  let cardsManagerAddr: string;

  before(async () => {
    client = getLocalSecretConnection();
    await ensureChainIsReady(client);
    [wallet] = getGenesisWallets();
  });

  it("Can deploy Cards Manager", async () => {
    const address = wallet.address;
    let signingClient = getLocalSecretConnection(wallet);
    cardsManagerAddr = await storeAndInitContract(
      signingClient,
      "../artifacts/cards_manager.wasm",
      { owner: address, message: "" },
      "cards_manager_v1"
    );
  });

  it("Can claim welcome pack", async () => {
    const address = wallet.address;
    let signingClient = getLocalSecretConnection(wallet);
    let res = await signingClient.tx.compute.executeContract(
      {
        contract_address: cardsManagerAddr,
        msg: { receive_welcome_pack: {} },
        sender: address,
      },
      { broadcastCheckIntervalMs: 200, gasLimit: 1000000 }
    );
    let ownedCards = await signingClient.query.compute.queryContract({
      contract_address: cardsManagerAddr,
      query: { owned_cards: { address: address } },
    });
    console.log(ownedCards);
  });
});
