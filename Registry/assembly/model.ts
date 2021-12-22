import { context, PersistentVector } from "near-sdk-as";

/**
 * Exporting a new class GameAddress so it can be used outside of this file.
 */
@nearBindgen
export class GameAddress {
  sender: string;
  constructor(public contractAddress: string, public name: string) {
    this.sender = context.sender;
  }
}
/**
 * collections.vector is a persistent collection. Any changes to it will
 * be automatically saved in the storage.
 * The parameter to the constructor needs to be unique across a single contract.
 * It will be used as a prefix to all keys required to store data in the storage.
 */
export const gameAddresses = new PersistentVector<GameAddress>("m");
