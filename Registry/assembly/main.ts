import { GameAddress, gameAddresses } from "./model";

// --- contract code goes below

// The maximum number of latest gameAddresses the contract returns.
const MESSAGE_LIMIT = 20;

/**
 * Adds a new gameAddress under the name of the sender's account id.\
 * NOTE: This is a change method. Which means it will modify the state.\
 * But right now we don't distinguish them with annotations yet.
 */
export function addGameAddress(text: string): void {
  // Creating a new gameAddress and populating fields with our data
  const gameAddress = new GameAddress(text);
  // Adding the gameAddress to end of the the persistent collection
  gameAddresses.push(gameAddress);
}

/**
 * Returns an array of last N gameAddresses.\
 * NOTE: This is a view method. Which means it should NOT modify the state.
 */
export function getGameAddresses(): GameAddress[] {
  const numGameAddresses = min(MESSAGE_LIMIT, gameAddresses.length);
  const startIndex = gameAddresses.length - numGameAddresses;
  const result = new Array<GameAddress>(numGameAddresses);
  for (let i = 0; i < numGameAddresses; i++) {
    result[i] = gameAddresses[i + startIndex];
  }
  return result;
}

/**
 * Returns an gameAddress by id.\
 * NOTE: This is a view method. Which means it should NOT modify the state.
 */
export function getGameAddress(id: i32): GameAddress {
  return gameAddresses[id];
}
